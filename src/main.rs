use actix_multipart::Multipart;
use actix_web::{
    web::{post, resource, Data},
    App, HttpResponse, HttpServer,
};
use form_data::{handle_multipart, Error, Field, FilenameGenerator, Form};
use futures::Future;
use std::path::PathBuf;
use uuid::Uuid;

struct FileNamer;

impl FilenameGenerator for FileNamer {
    fn next_filename(&self, _: &mime::Mime) -> Option<PathBuf> {
        let mut p = PathBuf::new();
        p.push(format!("uploaded-images/{}.jpg", Uuid::new_v4()));
        Some(p)
    }
}

fn upload(
    (mp, state): (Multipart, Data<Form>),
) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    Box::new(
        handle_multipart(mp, state.get_ref().clone()).map(|uploaded_content| {
            println!("Uploaded Content: {:?}", uploaded_content);
            HttpResponse::Created().finish()
        }),
    )
}

fn main() -> Result<(), failure::Error> {
    let form = Form::new().field("files", Field::array(Field::file(FileNamer)));

    println!("{:?}", form);

    HttpServer::new(move || {
        App::new()
            .data(form.clone())
            .service(resource("/upload").route(post().to(upload)))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap();

    Ok(())
}
