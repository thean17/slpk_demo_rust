mod cache;

use actix_web::{middleware, web, App, HttpServer};
use env_logger;
use log;
use sailfish::TemplateOnce;
use std::{self, sync::Mutex};

const CESIUM_ACCESS_TOKEN: &str = "";
const SLPK_FILENAME: &str = ".slpk";

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
struct Index {
    slpk_filename: String,
}

#[derive(TemplateOnce)]
#[template(path = "cesium.stpl")]
struct Cesium {
    access_token: String,
    slpk_filename: String,
}

fn get_scene_server_json(slpk_name: &str, layers: &str) -> String {
    let json: String = format!(
        r#"
        {{
            "serviceName": {},
            "name": {},
            "currentVersion": 10.6,
            "serviceVersion": "1.6",
            "supportedBindings": ["REST"],
            "layers": {}
        }}
        "#,
        slpk_name, slpk_name, layers
    );

    return json;
}

async fn get_scene_server(
    cache: web::Data<Mutex<cache::cache::Cache>>,
    path: web::Path<(String,)>,
) -> actix_web::HttpResponse {
    let mut cache = cache.lock().unwrap();
    let slpk_filename: Vec<&str> = path.0.split(".").collect();
    let scene_server_json = cache.read_file(&path.0, "3dSceneLayer.json.gz").unwrap();

    let mut response = actix_web::HttpResponse::Ok();

    let scene_server_json = String::from_utf8(scene_server_json).unwrap();

    response.content_type("application/json");

    return response.body(get_scene_server_json(
        slpk_filename.last().unwrap(),
        &scene_server_json,
    ));
}

async fn get_layer_0(
    cache: web::Data<Mutex<cache::cache::Cache>>,
    path: web::Path<(String,)>,
) -> actix_web::HttpResponse {
    let mut cache = cache.lock().unwrap();
    let scene_server_json = cache.read_file(&path.0, "3dSceneLayer.json.gz").unwrap();

    let mut response = actix_web::HttpResponse::Ok();

    let scene_server_json = String::from_utf8(scene_server_json).unwrap();

    response.content_type("application/json");

    return response.body(scene_server_json);
}

async fn get_node(
    cache: web::Data<Mutex<cache::cache::Cache>>,
    path: web::Path<(String, String)>,
) -> actix_web::HttpResponse {
    let node_index_document_json = {
        let mut cache = cache.lock().unwrap();
        let node_index_document_json = cache
            .read_file(
                &path.0,
                &format!("nodes/{}/3dNodeIndexDocument.json.gz", &path.1),
            )
            .unwrap();

        node_index_document_json
    };

    let mut response = actix_web::HttpResponse::Ok();

    let node_index_document_json = String::from_utf8(node_index_document_json).unwrap();

    response.content_type("application/json");

    return response.body(node_index_document_json);
}

async fn get_geometry(
    cache: web::Data<Mutex<cache::cache::Cache>>,
    path: web::Path<(String, String)>,
) -> actix_web::HttpResponse {
    let geometry_bin = {
        let mut cache = cache.lock().unwrap();

        let geometry_bin = cache
            .read_file(&path.0, &format!("nodes/{}/geometries/0.bin.gz", &path.1))
            .unwrap();

        geometry_bin
    };

    let mut response = actix_web::HttpResponse::Ok();

    response.content_type("application/octet-stream; charset=binary");

    return response.body(geometry_bin);
}

async fn get_texture_0_0(
    cache: web::Data<Mutex<cache::cache::Cache>>,
    path: web::Path<(String, String)>,
) -> actix_web::HttpResponse {
    let texture_0_0_jpg = {
        let mut cache = cache.lock().unwrap();

        let texture_0_0_jpg = cache
            .read_file(&path.0, &format!("nodes/{}/textures/0_0.jpg", &path.1))
            .unwrap();

        texture_0_0_jpg
    };

    let mut response = actix_web::HttpResponse::Ok();

    response.content_type("image/jpeg");
    response.append_header(("Content-Disposition", r#"attachment; filename="0_0.jpg""#));

    return response.body(texture_0_0_jpg);
}

async fn get_texture_0_0_1(
    cache: web::Data<Mutex<cache::cache::Cache>>,
    path: web::Path<(String, String)>,
) -> actix_web::HttpResponse {
    let texture_0_0_1_dds = {
        let mut cache = cache.lock().unwrap();

        let texture_0_0_1_dds = cache
            .read_file(
                &path.0,
                &format!("nodes/{}/textures/0_0_1.bin.dds.gz", &path.1),
            )
            .unwrap();

        texture_0_0_1_dds
    };

    let mut response = actix_web::HttpResponse::Ok();

    response.content_type("image/vnd-ms.dds");
    response.append_header((
        "Content-Disposition",
        r#"attachment; filename="0_0_1.bin.dds""#,
    ));

    return response.body(texture_0_0_1_dds);
}

async fn get_feature(
    cache: web::Data<Mutex<cache::cache::Cache>>,
    path: web::Path<(String, String)>,
) -> actix_web::HttpResponse {
    let feature_json = {
        let mut cache = cache.lock().unwrap();

        let feature_json = cache
            .read_file(&path.0, &format!("nodes/{}/features/0.json.gz", &path.1))
            .unwrap();

        feature_json
    };

    let mut response = actix_web::HttpResponse::Ok();

    response.content_type("application/json");

    return response.body(feature_json);
}

async fn get_shared_resource(
    cache: web::Data<Mutex<cache::cache::Cache>>,
    path: web::Path<(String, String)>,
) -> actix_web::HttpResponse {
    let shared_resource_json = {
        let mut cache = cache.lock().unwrap();

        let shared_resource_json = cache
            .read_file(
                &path.0,
                &format!("nodes/{}/shared/sharedResource.json.gz", &path.1),
            )
            .unwrap();

        shared_resource_json
    };

    let mut response = actix_web::HttpResponse::Ok();

    response.content_type("application/json");

    return response.body(shared_resource_json);
}

async fn get_cesium_page() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().body(
        Cesium {
            access_token: String::from(CESIUM_ACCESS_TOKEN),
            slpk_filename: String::from(SLPK_FILENAME),
        }
        .render_once()
        .unwrap(),
    )
}

async fn get_index_page() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().body(
        Index {
            slpk_filename: String::from(SLPK_FILENAME),
        }
        .render_once()
        .unwrap(),
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut global_cache = Mutex::new(cache::cache::Cache::new());

    let load_default_slpk_file = SLPK_FILENAME;

    global_cache
        .get_mut()
        .unwrap()
        .load_slpk(&format!("./slpk/{}", load_default_slpk_file));

    let cache = web::Data::new(global_cache);

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(cache.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .service(
                web::scope("/rest")
                    .service(
                        web::resource("/{slpk}/SceneServer").route(web::get().to(get_scene_server)),
                    )
                    .service(
                        web::resource("/{slpk}/SceneServer/layers/0")
                            .route(web::get().to(get_layer_0)),
                    )
                    .service(
                        web::resource("/{slpk}/SceneServer/layers/0/nodes/{node}")
                            .route(web::get().to(get_node)),
                    )
                    .service(
                        web::resource("/{slpk}/SceneServer/layers/0/nodes/{node}/geometries/0")
                            .route(web::get().to(get_geometry)),
                    )
                    .service(
                        web::resource("/{slpk}/SceneServer/layers/0/nodes/{node}/textures/0_0")
                            .route(web::get().to(get_texture_0_0)),
                    )
                    .service(
                        web::resource("/{slpk}/SceneServer/layers/0/nodes/{node}/textures/0_0_1")
                            .route(web::get().to(get_texture_0_0_1)),
                    )
                    .service(
                        web::resource("/{slpk}/SceneServer/layers/0/nodes/{node}/features/0")
                            .route(web::get().to(get_feature)),
                    )
                    .service(
                        web::resource("/{slpk}/SceneServer/layers/0/nodes/{node}/shared")
                            .route(web::get().to(get_shared_resource)),
                    ),
            )
            .service(web::resource("/cesium").route(web::get().to(get_cesium_page)))
            .service(web::resource("/").route(web::get().to(get_index_page)))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
