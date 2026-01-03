use rusqlite::{Statement, Transaction};

pub struct Statements<'a> {
    pub repo_insert: Statement<'a>,
    pub repo_check: Statement<'a>,
    pub package_insert: Statement<'a>,
    pub maintainer_insert: Statement<'a>,
    pub maintainer_check: Statement<'a>,
    pub pkg_maintainer_insert: Statement<'a>,
}

impl<'a> Statements<'a> {
    pub fn new(tx: &'a Transaction) -> rusqlite::Result<Self> {
        Ok(Self {
            repo_insert: tx.prepare("INSERT INTO repository (name, etag) VALUES (?1, ?2)")?,
            repo_check: tx.prepare("SELECT name FROM repository LIMIT 1")?,
            maintainer_insert: tx
                .prepare("INSERT INTO maintainers (name, contact) VALUES (?1, ?2)")?,
            maintainer_check: tx.prepare("SELECT id FROM maintainers WHERE contact=?1 LIMIT 1")?,
            pkg_maintainer_insert: tx.prepare(
                "INSERT INTO package_maintainers (
                        maintainer_id, package_id
                    ) VALUES (?1, ?2)
                    ON CONFLICT (maintainer_id, package_id) DO NOTHING",
            )?,
            package_insert: tx.prepare(
                "INSERT INTO packages (
                    rank, pkg_id, pkg_name, pkg_family, pkg_type, pkg_webpage,
                    app_id, description, version, licenses, download_url,
                    size, ghcr_pkg, ghcr_size, ghcr_files, ghcr_blob, ghcr_url,
                    bsum, icon, desktop, appstream, homepages, notes,
                    source_urls, tags, categories, build_id, build_date,
                    build_action, build_script, build_log, provides, snapshots,
                    repology, replaces, soar_syms, deprecated,
                    desktop_integration, portable, recurse_provides
                )
                VALUES
                (
                    ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, jsonb(?10),
                    ?11, ?12, ?13, ?14, jsonb(?15), ?16, ?17, ?18,
                    ?19, ?20, ?21, jsonb(?22), jsonb(?23), jsonb(?24),
                    jsonb(?25), jsonb(?26), ?27, ?28, ?29, ?30, ?31, jsonb(?32),
                    jsonb(?33), jsonb(?34), jsonb(?35), ?36, ?37, ?38, ?39,
                    ?40
                )
                ON CONFLICT (pkg_id, pkg_name, version) DO NOTHING",
            )?,
        })
    }
}
