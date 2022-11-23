variable "credentials_gcp" {
  type        = string
  description = "GCP Service Account for Terraform"
}

variable "project_id" {
  type        = string
  description = "GCP Project ID"
}

variable "region" {
  type        = string
  description = "GCP Region"
}

variable "container_tag" {
  type        = string
  description = "GCP Artefact Registry Container Tag"
  default     = "83eb945"
}
