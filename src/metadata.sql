CREATE TABLE repository (
  name TEXT NOT NULL UNIQUE,
  etag TEXT NOT NULL UNIQUE
);

CREATE TABLE maintainers (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  contact TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL
);

CREATE TABLE package_maintainers (
  maintainer_id INTEGER NOT NULL,
  package_id INTEGER NOT NULL,
  FOREIGN KEY (maintainer_id) REFERENCES packages (id),
  FOREIGN KEY (package_id) REFERENCES packages (id),
  UNIQUE (maintainer_id, package_id)
);

CREATE TABLE packages (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  disabled BOOLEAN NOT NULL DEFAULT false,
  disabled_reason JSONB,
  rank INT NOT NULL,
  pkg TEXT NOT NULL,
  pkg_id TEXT,
  pkg_name TEXT NOT NULL,
  pkg_family TEXT NOT NULL,
  pkg_type TEXT NOT NULL,
  pkg_webpage TEXT,
  app_id TEXT,
  description TEXT,
  version TEXT NOT NULL,
  version_upstream TEXT,
  licenses JSONB,
  download_url TEXT NOT NULL,
  size BIGINT NOT NULL,
  ghcr_pkg TEXT,
  ghcr_size BIGINT,
  ghcr_files JSONB,
  ghcr_blob TEXT,
  ghcr_url TEXT,
  bsum TEXT NOT NULL,
  shasum TEXT NOT NULL,
  icon TEXT,
  desktop TEXT,
  appstream TEXT,
  homepages JSONB,
  notes JSONB,
  source_urls JSONB,
  tags JSONB,
  categories JSONB,
  build_id TEXT,
  build_date TEXT,
  build_action TEXT,
  build_script TEXT,
  build_log TEXT,
  provides JSONB,
  snapshots JSONB,
  repology JSONB,
  download_count INTEGER,
  download_count_week INTEGER,
  download_count_month INTEGER,
  UNIQUE (pkg_id, pkg_name, version)
);
