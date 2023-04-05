use super::check_id_slug;
use crate::{
    structures::{project::*, Number},
    url_ext::{UrlJoinAll, UrlWithQuery},
    Ferinth, Result, API_BASE_URL,
};

impl Ferinth {
    /// Get the project of `project_id`
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> ferinth::Result<()> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// // Get a mod using its project ID
    /// let sodium = modrinth.get_project("AANobbMI").await?;
    /// assert_eq!(
    ///     sodium.title,
    ///     "Sodium",
    /// );
    ///
    /// // You can also use the project's slug
    /// let ok_zoomer = modrinth.get_project("ok-zoomer").await?;
    /// assert_eq!(ok_zoomer.title, "Ok Zoomer");
    /// # Ok(()) }
    /// ```
    pub async fn get_project(&self, project_id: &str) -> Result<Project> {
        check_id_slug(&[project_id])?;
        self.get(API_BASE_URL.join_all(vec!["project", project_id]))
            .await
    }

    /// Get multiple projects with IDs `project_ids`
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> ferinth::Result<()> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let mods = modrinth.get_multiple_projects(&[
    ///     "AANobbMI",
    ///     "P7dR8mSH",
    ///     "gvQqBUqZ",
    ///     "YL57xq9U",
    /// ]).await?;
    /// assert_eq!(mods.len(), 4);
    /// # Ok(()) }
    /// ```
    pub async fn get_multiple_projects(&self, project_ids: &[&str]) -> Result<Vec<Project>> {
        check_id_slug(project_ids)?;
        self.get(
            API_BASE_URL
                .join_all(vec!["projects"])
                .with_query(&[("ids", &serde_json::to_string(project_ids)?)]),
        )
        .await
    }

    /// Get `count` number of random projects
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> ferinth::Result<()> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let random_mods = modrinth.get_random_projects(5).await?;
    /// assert_eq!(random_mods.len(), 5);
    /// # Ok(()) }
    /// ```
    pub async fn get_random_projects(&self, count: Number) -> Result<Vec<Project>> {
        self.get(
            API_BASE_URL
                .join_all(vec!["projects_random"])
                .with_query(&[("count", &count.to_string())]),
        )
        .await
    }

    /// Check if the given ID or slug refers to an existing project.
    /// If so, the ID of the project will be returned.
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> ferinth::Result<()> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let project_id = modrinth.does_exist("sodium").await?;
    /// assert_eq!(project_id, "AANobbMI");
    /// # Ok(()) }
    /// ```
    pub async fn does_exist(&self, project_id: &str) -> Result<String> {
        #[derive(serde::Deserialize)]
        struct Response {
            id: String,
        }
        check_id_slug(&[project_id])?;
        let res: Response = self
            .get(API_BASE_URL.join_all(vec!["project", project_id, "check"]))
            .await?;
        Ok(res.id)
    }

    /// Add the given gallery `image`, with a file `ext`ention and an optional `title`, to `project_id`.
    /// State whether the image should be `featured` or not, and optionally provide a `description`.
    ///
    /// The image data can have a maximum size of `5 MiB`
    ///
    /// REQUIRES AUTHENTICATION!
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new(
    /// #     env!("CARGO_CRATE_NAME"),
    /// #     Some(env!("CARGO_PKG_VERSION")),
    /// #     None,
    /// #     Some(env!("MODRINTH_TOKEN")),
    /// # )?;
    /// # let project_id = env!("TEST_PROJECT_ID");
    /// # let image_data = &std::fs::read("test_image.png").expect("Failed to read test image");
    /// modrinth.add_gallery_image(
    ///     project_id,
    ///     image_data,
    ///     ferinth::structures::project::FileExt::PNG,
    ///     true,
    ///     Some("Test image".to_string()),
    ///     Some("This is a test image".to_string()),
    /// ).await?;
    /// # Ok(()) }
    /// ```
    pub async fn add_gallery_image(
        &self,
        project_id: &str,
        image: &[u8],
        ext: FileExt,
        featured: bool,
        title: Option<String>,
        description: Option<String>,
    ) -> Result<()> {
        check_id_slug(&[project_id])?;
        let mut query = vec![("ext", ext.to_string()), ("featured", featured.to_string())];
        if let Some(title) = title {
            query.push(("title", title));
        }
        if let Some(description) = description {
            query.push(("description", description));
        }
        self.post(
            API_BASE_URL
                .join_all(vec!["project", project_id, "gallery"])
                .with_query(query),
            image.to_vec(),
            &format!("image/{}", ext),
        )
        .await?;
        Ok(())
    }

    /// Get the dependencies of the project of `project_id`
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> ferinth::Result<()> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let fabric_api = modrinth.get_project_dependencies("fabric-api").await?;
    /// // Fabric API should not have any dependencies
    /// assert!(fabric_api.projects.is_empty());
    /// # Ok(()) }
    /// ```
    pub async fn get_project_dependencies(&self, project_id: &str) -> Result<ProjectDependencies> {
        check_id_slug(&[project_id])?;
        self.get(API_BASE_URL.join_all(vec!["project", project_id, "dependencies"]))
            .await
    }

    /// Follow the project of `project_id`
    ///
    /// REQUIRES AUTHENTICATION!
    ///
    /// ```ignore
    /// modrinth.follow(project_id).await?;
    /// ```
    pub async fn follow(&self, project_id: &str) -> Result<()> {
        check_id_slug(&[project_id])?;
        self.post_json(
            API_BASE_URL.join_all(vec!["project", project_id, "follow"]),
            "",
        )
        .await
    }

    /// Unfollow the project of `project_id`
    ///
    /// REQUIRES AUTHENTICATION!
    ///
    /// ```ignore
    /// modrinth.unfollow(project_id).await?;
    /// ```
    pub async fn unfollow(&self, project_id: &str) -> Result<()> {
        check_id_slug(&[project_id])?;
        self.delete(API_BASE_URL.join_all(vec!["project", project_id, "follow"]))
            .await?;
        Ok(())
    }
}
