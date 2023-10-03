locals {
	has_minio = can(var.s3_providers["minio"])
	service_minio = lookup(var.services, "minio", {
		count = 1
		resources = {
			cpu = 50
			cpu_cores = 0
			memory = 400
		}
	})
}

resource "kubernetes_namespace" "minio" {
	count = local.has_minio ? 1 : 0

	metadata {
		name = "minio"
	}
}

module "minio_secrets" {
	count = local.has_minio ? 1 : 0

	source = "../modules/secrets"

	keys = ["s3/minio/root/key_id", "s3/minio/root/key"]
	optional = true
}

resource "helm_release" "minio" {
	depends_on = [helm_release.prometheus]
	count = local.has_minio ? 1 : 0

	name = "minio"
	namespace = kubernetes_namespace.minio[0].metadata.0.name
	repository = "oci://registry-1.docker.io/bitnamicharts"
	chart = "minio"
	version = "12.8.3"
	values = [yamlencode({
		global = {
			storageClass = var.k8s_storage_class
		}
		replicaCount = local.service_minio.count
		resources = {
			limits = {
				memory = "${local.service_minio.resources.memory}Mi"
				cpu = (
					local.service_minio.resources.cpu_cores > 0 ?
					"${local.service_minio.resources.cpu_cores * 1000}m"
					: "${local.service_minio.resources.cpu}m"
				)
			}
		}

		auth = {
			rootUser = module.minio_secrets[0].values["s3/minio/root/key_id"]
			rootPassword = module.minio_secrets[0].values["s3/minio/root/key"]
		}
		service = {
			# Expose as LB so it can be accessed from the host if needed
			type = var.minio_port != null ? "LoadBalancer" : "ClusterIP"
		}
		metrics = {
			serviceMonitor = {
				enabled = true
				namespace = kubernetes_namespace.minio[0].metadata.0.name
			}

			# TODO:
			# prometheusRule = {
			# 	enabled = true
			# 	namespace = kubernetes_namespace.prometheus.metadata.0.name
			# }
		}
	})]
}

# TODO: Errors if minio isn't enabled in namespace config
resource "kubectl_manifest" "minio_ingress_route" {
	# Expose via Traefik if not using Minio port
	for_each = var.minio_port == null ? local.entrypoints : {}

	depends_on = [helm_release.minio]

	yaml_body = yamlencode({
		apiVersion = "traefik.containo.us/v1alpha1"
		kind = "IngressRoute"

		metadata = {
			name = "minio"
			namespace = kubernetes_namespace.minio[0].metadata.0.name
		}

		spec = {
			entryPoints = [ each.key ]

			routes = [
				{
					match = "Host(`minio.${var.domain_main}`)"
					kind  = "Rule"
					services = [
						{
							name = "minio"
							port = 9000
						}
					]
				}
			]

			tls = each.value.tls
		}
	})
}
