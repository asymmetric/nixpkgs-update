use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::packages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Package {
    pub id: String,
    pub attr_path: String,
    pub version_nixpkgs_master: Option<String>,
    pub version_nixpkgs_staging: Option<String>,
    pub version_nixpkgs_staging_next: Option<String>,
    pub version_repology: Option<String>,
    pub version_github: Option<String>,
    pub version_gitlab: Option<String>,
    pub version_pypi: Option<String>,
    pub project_repology: Option<String>,
    pub nixpkgs_name_replogy: Option<String>,
    pub owner_github: Option<String>,
    pub repo_github: Option<String>,
    pub owner_gitlab: Option<String>,
    pub repo_gitlab: Option<String>,
    pub last_checked_repology: Option<String>,
    pub last_checked_github: Option<String>,
    pub last_hecked_gitlab: Option<String>,
    pub last_hecked_pypi: Option<String>,
    pub last_checked_pending_pr: Option<String>,
    pub last_update_attempt: Option<String>,
    pub pending_pr: Option<i32>,
    pub pending_pr_owner: Option<String>,
    pub pending_pr_branch_name: Option<String>,
    pub last_update_log: Option<String>,

}
