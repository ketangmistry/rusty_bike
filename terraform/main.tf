provider "google" {
  credentials = var.credentials_gcp
  project     = var.project_id
}

module "serverless" {
  source     = "./modules/serverless"
  project_id = var.project_id
  region     = var.region
}