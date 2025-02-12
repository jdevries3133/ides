terraform {

  backend "s3" {
    bucket = "my-sites-terraform-remote-state"
    key    = "ides-2"
    region = "us-east-2"
  }

  required_providers {
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = ">= 2.7.1"
    }
    helm = {
      source  = "hashicorp/helm"
      version = ">= 2.4.1"
    }
  }
}

provider "kubernetes" {
  config_path = "~/.kube/config"
}

provider "helm" {
  kubernetes {
    config_path = "~/.kube/config"
  }
}

variable "smtp_email_password" {
  type      = string
  sensitive = true
}

data "external" "git_sha" {
  program = [
    "sh",
    "-c",
    "echo '{\"output\": \"'\"$(if [[ ! -z $GITLAB_SHA ]]; then echo $GITLAB_SHA; else git rev-parse HEAD; fi)\"'\"}'"
  ]
}

module "basic-deployment" {
  source  = "jdevries3133/basic-deployment/kubernetes"
  version = "3.2.0"

  app_name  = "phat-stack"
  container = "jdevries3133/ides:${data.external.git_sha.result.output}"
  domain    = "ides.katetell.com"

  extra_env = {
    SMTP_EMAIL_USERNAME           = "jdevries3133@gmail.com"
    SMTP_EMAIL_PASSWORD           = var.smtp_email_password
  }
}
