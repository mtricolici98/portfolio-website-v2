#[derive(PartialEq, Clone, Debug)]
pub struct GalleryElementData {
    pub name: String,
    pub desctiption: String,
    pub main_image_url: String,
    pub images_url: Vec<String>,
    pub video_url: Option<String>,
}



pub fn get_gallery_data() -> Vec<GalleryElementData> {
    vec![
        GalleryElementData {
            name: String::from("Interactivity"),
            desctiption: String::from("Complex animations and interactivity with your presentations..."),
            main_image_url: String::from("img/gallery/browser-like.png"),
            images_url: vec![
                String::from("img/gallery/browser-like.png"),
                String::from("img/gallery/browser-like.png"),
                String::from("img/gallery/browser-like.png"),
                String::from("img/gallery/browser-like.png"),
                String::from("img/gallery/browser-like.png"),
                String::from("img/gallery/browser-like.png")
            ],
            video_url: Some(String::from("img/gallery/browser-like.webm")),
        },
        GalleryElementData {
            name: String::from("Interactivity"),
            desctiption: String::from("Complex animations and interactivity with your presentations..."),
            main_image_url: String::from("img/gallery/p1_i1.png"),
            images_url: vec![],
            video_url: Some(String::from("img/gallery/p1_m.gif")),
        },
        GalleryElementData {
            name: String::from("Interactivity"),
            desctiption: String::from("Complex animations and interactivity with your presentations..."),
            main_image_url: String::from("img/gallery/c_thumb.png"),
            images_url: vec![],
            video_url: Some(String::from("img/gallery/cancer.webm")),
        },
        GalleryElementData {
            name: String::from("Interactivity"),
            desctiption: String::from("Complex animations and interactivity with your presentations..."),
            main_image_url: String::from("img/gallery/browser-like.png"),
            images_url: vec![],
            video_url: Some(String::from("img/gallery/browser-like.webm")),
        },
    ]
}