use domain::command::git_log::{GitLogCommandOption, GitLogCommandOutput, GitLogCommandTrait};

pub struct GitLogCommandNormalFixture;
impl GitLogCommandTrait for GitLogCommandNormalFixture {
    fn new() -> Self {
        Self
    }
    fn execute(&self, option: GitLogCommandOption) -> GitLogCommandOutput {
        let output = r#"
0cbdaf6b5211d36df10a578b621e63eab3bc52a4 6bec089d8d748eb1a800955bac06597f1fd22414 19c871de65be6334404cb7d26c7b7bed04ad8c00 [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 22:34:51 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 22:34:51 +0900] [git logの具体的なパーザーを実装]
19c871de65be6334404cb7d26c7b7bed04ad8c00 342527695ded619de3a3518c01d1f30a1cc54880 4815d86c681aa3e5dfa81c23ecff6fbab13900eb [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 21:51:24 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 21:51:24 +0900] [git logのfixtureを作成]
4815d86c681aa3e5dfa81c23ecff6fbab13900eb e2d5c8dc9bf84f007f2dba57d251778a1ebcd4f5 8efe09751fe292928bcc17c500f053d2ea163854 [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 21:16:55 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 21:16:55 +0900] [git_branchの具体的な実装を追加]
8efe09751fe292928bcc17c500f053d2ea163854 aa4aea647280756db8a6ac774d87b19ec622561c 759dec1a7f524354ece51b4ba0dd51ab7d4288f6 [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 20:43:32 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 20:43:32 +0900] [git_logコマンドの部品を製造中]
759dec1a7f524354ece51b4ba0dd51ab7d4288f6 bb4b2b873697f3ecdac554d30c2424fd46097b3c a361aa46969ae6c4f1a6f56cd8a9240f496d33cd [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 20:11:27 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 20:11:27 +0900] [名前をリファクタ]
a361aa46969ae6c4f1a6f56cd8a9240f496d33cd 0420b606e8818d751b71872511beb42d58e25770 79d7659e5846952dfba6b15ce21cbed152ad2b0d [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 20:10:12 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 20:10:12 +0900] [テストケース名作成]
79d7659e5846952dfba6b15ce21cbed152ad2b0d f07ba9a746b38f536d6df1669a8880ee0f193966 fe901f10d5275555d799890c798961c01d7662ec [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 19:53:08 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 19:53:08 +0900] [作成したワークスペースを呼び出せるように変更]
fe901f10d5275555d799890c798961c01d7662ec 94f61bc573b1194d98908cbca2e02a97f174c6d9 cd281c1869f874907fab4f772ade30263202818d [t.onodera] t.onodera.1.2.1.5@gmail.com [Thu, 26 Jan 2023 21:55:07 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Thu, 26 Jan 2023 21:55:07 +0900] [ベースができた！]
cd281c1869f874907fab4f772ade30263202818d e087f3395a6ff2c76a9b40068470d2d8aefb6a8d 84c97fdb36856cfd55332906872218b440e5bff0 [t.onodera] t.onodera.1.2.1.5@gmail.com [Thu, 26 Jan 2023 21:18:09 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Thu, 26 Jan 2023 21:18:09 +0900] [ドメインサービス追加]
84c97fdb36856cfd55332906872218b440e5bff0 f95c36bd6d35dd861fae9e83089c724a364b8c16 dbda4a1409c9a439b547b0e7f0efa4b983a38a60 [t.onodera] t.onodera.1.2.1.5@gmail.com [Thu, 26 Jan 2023 21:08:07 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Thu, 26 Jan 2023 21:08:07 +0900] [ファイル分割]
dbda4a1409c9a439b547b0e7f0efa4b983a38a60 ee5f06e41338400757e9cb8698360de93a4c2c12 99eeab74750f9d3098d3bef3681145e257545c9f [t.onodera] t.onodera.1.2.1.5@gmail.com [Thu, 26 Jan 2023 00:05:50 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Thu, 26 Jan 2023 00:05:50 +0900] [いったんコミット]
99eeab74750f9d3098d3bef3681145e257545c9f f6adf0012fd2a27663af2e94b36de41cbba5823e  [t.onodera] t.onodera.1.2.1.5@gmail.com [Tue, 24 Jan 2023 21:19:56 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Tue, 24 Jan 2023 21:19:56 +0900] [最初のコミット]
"#
        .as_bytes()
        .to_vec();
        Ok(output)
    }
}
pub struct GitLogCommandErrorFixture1;
impl GitLogCommandTrait for GitLogCommandErrorFixture1 {
    fn new() -> Self {
        Self
    }
    fn execute(&self, option: GitLogCommandOption) -> GitLogCommandOutput {
        // author_date,commit_dateがないデータは不正扱い
        let output = r#"
0cbdaf6b5211d36df10a578b621e63eab3bc52a4 6bec089d8d748eb1a800955bac06597f1fd22414 19c871de65be6334404cb7d26c7b7bed04ad8c00 [t.onodera] t.onodera.1.2.1.5@gmail.com [] [t.onodera] t.onodera.1.2.1.5@gmail.com [] [git logの具体的なパーザーを実装]
19c871de65be6334404cb7d26c7b7bed04ad8c00 342527695ded619de3a3518c01d1f30a1cc54880 4815d86c681aa3e5dfa81c23ecff6fbab13900eb [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 21:51:24 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 21:51:24 +0900] [git logのfixtureを作成]
4815d86c681aa3e5dfa81c23ecff6fbab13900eb e2d5c8dc9bf84f007f2dba57d251778a1ebcd4f5 8efe09751fe292928bcc17c500f053d2ea163854 [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 21:16:55 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 21:16:55 +0900] [git_branchの具体的な実装を追加]
8efe09751fe292928bcc17c500f053d2ea163854 aa4aea647280756db8a6ac774d87b19ec622561c 759dec1a7f524354ece51b4ba0dd51ab7d4288f6 [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 20:43:32 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 20:43:32 +0900] [git_logコマンドの部品を製造中]
759dec1a7f524354ece51b4ba0dd51ab7d4288f6 bb4b2b873697f3ecdac554d30c2424fd46097b3c a361aa46969ae6c4f1a6f56cd8a9240f496d33cd [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 20:11:27 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 20:11:27 +0900] [名前をリファクタ]
a361aa46969ae6c4f1a6f56cd8a9240f496d33cd 0420b606e8818d751b71872511beb42d58e25770 79d7659e5846952dfba6b15ce21cbed152ad2b0d [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 20:10:12 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 20:10:12 +0900] [テストケース名作成]
79d7659e5846952dfba6b15ce21cbed152ad2b0d f07ba9a746b38f536d6df1669a8880ee0f193966 fe901f10d5275555d799890c798961c01d7662ec [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 19:53:08 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Fri, 27 Jan 2023 19:53:08 +0900] [作成したワークスペースを呼び出せるように変更]
fe901f10d5275555d799890c798961c01d7662ec 94f61bc573b1194d98908cbca2e02a97f174c6d9 cd281c1869f874907fab4f772ade30263202818d [t.onodera] t.onodera.1.2.1.5@gmail.com [Thu, 26 Jan 2023 21:55:07 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Thu, 26 Jan 2023 21:55:07 +0900] [ベースができた！]
cd281c1869f874907fab4f772ade30263202818d e087f3395a6ff2c76a9b40068470d2d8aefb6a8d 84c97fdb36856cfd55332906872218b440e5bff0 [t.onodera] t.onodera.1.2.1.5@gmail.com [Thu, 26 Jan 2023 21:18:09 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Thu, 26 Jan 2023 21:18:09 +0900] [ドメインサービス追加]
84c97fdb36856cfd55332906872218b440e5bff0 f95c36bd6d35dd861fae9e83089c724a364b8c16 dbda4a1409c9a439b547b0e7f0efa4b983a38a60 [t.onodera] t.onodera.1.2.1.5@gmail.com [Thu, 26 Jan 2023 21:08:07 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Thu, 26 Jan 2023 21:08:07 +0900] [ファイル分割]
dbda4a1409c9a439b547b0e7f0efa4b983a38a60 ee5f06e41338400757e9cb8698360de93a4c2c12 99eeab74750f9d3098d3bef3681145e257545c9f [t.onodera] t.onodera.1.2.1.5@gmail.com [Thu, 26 Jan 2023 00:05:50 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Thu, 26 Jan 2023 00:05:50 +0900] [いったんコミット]
99eeab74750f9d3098d3bef3681145e257545c9f f6adf0012fd2a27663af2e94b36de41cbba5823e  [t.onodera] t.onodera.1.2.1.5@gmail.com [Tue, 24 Jan 2023 21:19:56 +0900] [t.onodera] t.onodera.1.2.1.5@gmail.com [Tue, 24 Jan 2023 21:19:56 +0900] [最初のコミット]
"#
        .as_bytes()
        .to_vec();
        Ok(output)
    }
}
