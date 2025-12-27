provider "google" {
  project = "your-project-id"
  region  = "us-central1"
}

# 1. Cloud Storage for Models & Assets
resource "google_storage_bucket" "antigravity_assets" {
  name          = "antigravity-hybrid-assets"
  location      = "US"
  force_destroy = true

  uniform_bucket_level_access = true
}

# 2. Firestore Database for Presets
resource "google_firestore_database" "presets_db" {
  name        = "(default)"
  location_id = "nam5"
  type        = "FIRESTORE_NATIVE"
}

# 3. Cloud Function for Licensing & Sync (Placeholder)
# In a real setup, we'd zip the source code and deploy it here.
resource "google_cloudfunctions_function" "sync_function" {
  name        = "antigravity-sync"
  description = "Handles preset sync and license checks"
  runtime     = "nodejs16"

  available_memory_mb   = 128
  source_archive_bucket = google_storage_bucket.antigravity_assets.name
  source_archive_object = "functions/sync.zip" # Placeholder path
  trigger_http          = true
  entry_point           = "syncHandler"
}

# 4. IAM Entry (Public Invoker for testing, restrict in prod)
resource "google_cloudfunctions_function_iam_member" "invoker" {
  project        = google_cloudfunctions_function.sync_function.project
  region         = google_cloudfunctions_function.sync_function.region
  cloud_function = google_cloudfunctions_function.sync_function.name

  role   = "roles/cloudfunctions.invoker"
  member = "allUsers"
}
