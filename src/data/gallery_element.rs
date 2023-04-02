use std::collections::{BTreeMap};

#[derive(PartialEq, Clone, Debug)]
pub struct GalleryElementData {
    pub name: String,
    pub desctiption: String,
    pub main_image_url: String,
    pub images_url: Vec<String>,
    pub video_url: Option<String>,
}


#[derive(PartialEq, Clone, Debug, Eq, Hash, PartialOrd, Ord)]
pub struct GalleryElementsCategory {
    pub name: String,
    pub description: String,
    pub price: String,
}





pub fn get_gallery_data_simple() -> Vec<GalleryElementData> {
    vec![
        GalleryElementData {
            name: String::from("Elegance"),
            desctiption: String::from("Get your point across, nothing extra"),
            main_image_url: String::from("img/gallery/med-cancer/cancer1.png"),
            images_url: vec![
                String::from("img/gallery/med-cancer/cancer1.png"),
                String::from("img/gallery/med-cancer/cancer2.png"),
                String::from("img/gallery/med-cancer/cancer3.png"),
                String::from("img/gallery/med-cancer/cancer4.png"),
                String::from("img/gallery/med-cancer/cancer5.png"),
                String::from("img/gallery/med-cancer/cancer6.png"),
                String::from("img/gallery/med-cancer/cancer7.png"),
                String::from("img/gallery/med-cancer/cancer8.png"),
                String::from("img/gallery/med-cancer/cancer9.png"),
            ],
            video_url: Some(String::from("img/gallery/med-cancer/cancer.webm")),
        },
        GalleryElementData {
            name: String::from("Practical"),
            desctiption: String::from("Your information, right where you want it..."),
            main_image_url: String::from("img/gallery/simple-med/med2.png"),
            images_url: vec![
                String::from("img/gallery/simple-med/med1.png"),
                String::from("img/gallery/simple-med/med2.png"),
                String::from("img/gallery/simple-med/med3.png"),
                String::from("img/gallery/simple-med/med4.png"),
                String::from("img/gallery/simple-med/med5.png"),
                String::from("img/gallery/simple-med/med6.png"),
                String::from("img/gallery/simple-med/med7.png"),
                String::from("img/gallery/simple-med/med8.png"),
                String::from("img/gallery/simple-med/med9.png"),
                String::from("img/gallery/simple-med/med10.png"),
                String::from("img/gallery/simple-med/med11.png"),
                String::from("img/gallery/simple-med/med12.png"),
                String::from("img/gallery/simple-med/med13.png"),
                String::from("img/gallery/simple-med/med14.png"),
                String::from("img/gallery/simple-med/med15.png"),
                String::from("img/gallery/simple-med/med16.png"),
            ],
            video_url: None,
        },
    ]
}



pub fn get_gallery_data_advanced() -> Vec<GalleryElementData> {
    vec![
        GalleryElementData {
            name: String::from("Pitch Deck"),
            desctiption: String::from("Built to tailor to your needs"),
            main_image_url: String::from("img/gallery/business/business2.png"),
            images_url: vec![
                String::from("img/gallery/business/business1.png"),
                String::from("img/gallery/business/business2.png"),
                String::from("img/gallery/business/business3.png"),
                String::from("img/gallery/business/business4.png"),
                String::from("img/gallery/business/business5.png"),
                String::from("img/gallery/business/business6.png"),
                String::from("img/gallery/business/business7.png"),
                String::from("img/gallery/business/business8.png"),
                String::from("img/gallery/business/business9.png"),
                String::from("img/gallery/business/business10.png"),
                String::from("img/gallery/business/business11.png"),
                String::from("img/gallery/business/business12.png"),
                String::from("img/gallery/business/business13.png"),
                String::from("img/gallery/business/business14.png"),
                String::from("img/gallery/business/business15.png"),
                String::from("img/gallery/business/business16.png"),
                String::from("img/gallery/business/business17.png"),
            ],
            video_url: Some(String::from("img/gallery/business/business.webm")),
        },
        GalleryElementData {
            name: String::from("Storybouard"),
            desctiption: String::from("A coherent story, to share with everyone"),
            main_image_url: String::from("img/gallery/storyboard/storyboard2.png"),
            images_url: vec![
                String::from("img/gallery/storyboard/storyboard1.png"),
                String::from("img/gallery/storyboard/storyboard2.png"),
                String::from("img/gallery/storyboard/storyboard3.png"),
                String::from("img/gallery/storyboard/storyboard4.png"),
                String::from("img/gallery/storyboard/storyboard5.png"),
                String::from("img/gallery/storyboard/storyboard6.png"),
                String::from("img/gallery/storyboard/storyboard7.png"),
                String::from("img/gallery/storyboard/storyboard8.png"),
            ],
            video_url: Some(String::from("img/gallery/storyboard/storyboard.mp4")),
        },
    ]
}



pub fn get_gallery_data_complex() -> Vec<GalleryElementData> {
    vec![
        GalleryElementData {
            name: String::from("Interactivity"),
            desctiption: String::from("Complex animations and interactivity with your presentations..."),
            main_image_url: String::from("img/gallery/web-like/web_like2.png"),
            images_url: vec![
                String::from("img/gallery/web-like/web_like1.png"),
                String::from("img/gallery/web-like/web_like2.png"),
                String::from("img/gallery/web-like/web_like3.png"),
                String::from("img/gallery/web-like/web_like4.png"),
                String::from("img/gallery/web-like/web_like5.png"),
                String::from("img/gallery/web-like/web_like6.png"),
                String::from("img/gallery/web-like/web_like7.png"),
                ],
            video_url: Some(String::from("img/gallery/web-like/browser-like.webm")),
        },
        GalleryElementData {
            name: String::from("Eye candy"),
            desctiption: String::from("Have a bit of lush while sharing your ideas..."),
            main_image_url: String::from("img/gallery/graduation/graduation2.png"),
            images_url: vec![
                String::from("img/gallery//graduation/graduation1.png"),
                String::from("img/gallery//graduation/graduation2.png"),
                String::from("img/gallery//graduation/graduation3.png"),
                String::from("img/gallery//graduation/graduation4.png"),
                String::from("img/gallery//graduation/graduation5.png"),
                String::from("img/gallery//graduation/graduation6.png"),
                ],
            video_url: Some(String::from("img/gallery/graduation/graduation.mp4")),
        },
    ]
}



pub fn get_gallery_data_map() -> BTreeMap<GalleryElementsCategory, Vec<GalleryElementData>> {
    let map:BTreeMap<GalleryElementsCategory, Vec<GalleryElementData>> = BTreeMap::from(
        [
            (
                GalleryElementsCategory{
                     name: String::from("Simple"),
                     description: String::from("Well thought out text, image and colour combinations, eye catchy but with minimal animations and interactivity"),
                     price: String::from("~ 1.5$/slide")
                },
                get_gallery_data_simple()
            ),(
                GalleryElementsCategory{
                     name: String::from("Advanced"),
                     description: String::from("A \"Complete package\", tailored for your need."),
                     price: String::from("~ 2$/slide")
                },
                get_gallery_data_advanced()
            ),(
                GalleryElementsCategory{
                     name: String::from("Complex"),
                     description: String::from("\"WOW IS THIS EVEN POWERPOINT ?\""),
                     price: String::from("~ 2.5$/slide")
                },
                get_gallery_data_complex()
            ),
        ]
    );
    map
}