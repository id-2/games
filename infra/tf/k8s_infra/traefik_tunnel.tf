locals {
	# Specify what services to expose via the tunnel server
	tunnel_services = {
		"nomad" = {
			service = "nomad-server"
			service_namespace = kubernetes_namespace.nomad.metadata[0].name
			service_port = 4647
		}
		"api-route" = {
			service = "rivet-api-route"
			service_namespace = kubernetes_namespace.rivet_service.metadata[0].name
			service_port = 80
		}
		"vector" = {
			service = "vector"
			service_namespace = kubernetes_namespace.vector.metadata[0].name
			service_port = 6000
		}
	}
}

resource "kubernetes_namespace" "traefik_tunnel" {
	metadata {
		name = "traefik-tunnel"
	}
}

# need to adjust config for second instance of traefik? 
resource "helm_release" "traefik_tunnel" {
	name = "traefik-tunnel"
	namespace = kubernetes_namespace.traefik_tunnel.metadata.0.name

	repository = "https://traefik.github.io/charts"
	chart = "traefik"
	version = "24.0.0"
	values = [yamlencode({
		# Use Traefik v3 beta for TLS servers transport support
		image = {
			tag = "v3.0.0-beta3"
		}
		ports = {
			# Disable default ports
			web = {
				expose = false
			},
			websecure = {
				expose = false
			},

			# Expose tunnel
			tunnel = {
				port = 5000
				expose = true
				exposedPort = 5000
				protocol = "TCP"
				tls = {
					enabled = true
					options = "ingress-tunnel"
				}
			}
		}

		tlsOptions = {
			"ingress-tunnel" = {
				curvePreferences = [ "CurveP384" ]

				clientAuth = {
					secretNames = [ "ingress-tls-ca-cert-locally-signed" ]
					clientAuthType = "RequireAndVerifyClientCert"
				}
			}
		}

		# Allows referencing services outside of the traefik namespace
		# TODO eventually just specify the namespace(s) that are relevant so that not pulling in configs unncessarily
		providers = {
			kubernetesCRD = {
				allowCrossNamespace = true
				labelSelector = "traefik-instance=tunnel"
			}
		}

		commonLabels = {
			"traefik-instance" = "tunnel"
		}

		logs = {
			general = {
				level = "DEBUG"
			}
			access = {
				enabled = true
			}
		}

		metrics = {
			prometheus = {
				addEntryPointsLabels = true
				addRoutersLabels = true
				addServicesLabels = true
				# See lib/chirp/metrics/src/buckets.rs
				buckets = "0.001,0.0025,0.005,0.01,0.025,0.05,0.1,0.25,0.5,1.0,2.5,5.0,10.0,25.0,50.0,100.0"
			}
		}
	})]
}

data "kubernetes_service" "traefik_tunnel" {
	depends_on = [helm_release.traefik_tunnel]

	metadata {
		name = "traefik"
		namespace = kubernetes_namespace.traefik_tunnel.metadata.0.name
	}
}

resource "kubectl_manifest" "traefik_nomad_router" {
	depends_on = [helm_release.traefik_tunnel]

	for_each = local.tunnel_services

	yaml_body = yamlencode({
		apiVersion = "traefik.io/v1alpha1"
		kind = "IngressRouteTCP"

		metadata = {
			name = each.key
			namespace = each.value.service_namespace
			labels = {
				"traefik-instance" = "tunnel"
			}
		}

		spec = {
			entryPoints = ["tunnel"]

			routes = [
				{
					kind = "Rule"
					match = "HostSNI(`${each.key}.tunnel.rivet.gg`)"
					services = [
						{
							name = each.value.service
							port = each.value.service_port
							# Directly access the service
							nativeLB = true
						}
					]
				}
			]

			tls = {
				secretName = "ingress-tls-cert-tunnel-server"
				options = {
					name = "ingress-tunnel",
					namespace = "traefik-tunnel"
				}

			}
		}
	})
}

