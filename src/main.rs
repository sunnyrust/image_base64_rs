use image::DynamicImage;

fn main(){
    let image_bytes=image_base64_rs::base64_to_vec("data:image/jpeg;base64,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3K3dt/mfhlWvUqW9pJv1dye1urmyuYr2yle3uLd1liliYpJHIhyrKwwVZSMgjkGtUNr/AIx1+CKWefVNW1SeG2R7mYvLNK5WKNWklb/dUFmwBjkAVh1Pa3VzZXMV7ZSvb3Fu6yxSxMUkjkQ5VlYYKspGQRyDXJKP2o/ETGX2ZbHqnxP8FfF34NeKn8FfEkXmj61HBFctbf2hHdYimBKN5ltNLHyAeN2R3FcRe+MvF+pSifUdc1K6kVQgee7mkYKCTgFmJxknj3pvijxd4s8b6q2u+NNa1HX9SZFia81S7lvLgxp91TLMzvtXsM4Fc7XTTx2NfJUr1W6ivqm9L72u21fS+p11sbVc26U5W6XeppXGs6xdwtb3V9czRPjckkzspwcjIJwcEZrNooorYirWlzVZOT83f8zlqVZ1Heo235hRRRWJmFFFFABRRRQAUUUUAFFFFABRRRQAUUUUAFFFFABRRRQB/9D8p6KKK/og/BwooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigD/0fynooor+iD8HCiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooA9l+B/wAd/H/7Pfiy78afDmW1i1K906TS5WvIBcR/Z5ZYZmAUkYbfAnPpkd6+qP8Ah55+1P8A8/uh/wDgrT/4uvzyorzsTlGDxE/aV6acu56GHzXF0Iezo1Gkf//S/Keiiiv6IPwcKKKKACiiigAooooAKKKKACiiigArqfDPgfxr40+2/wDCHeH9U13+zYftN7/ZllNefZoBn95N5KP5acH5mwOOtHgfwz/wmnjXw/4O+2w6b/buqWWmfbbniC2+2TJD50nI+SPfubkcA1+t3wy+GX/DF/8AbX/E6s/iR/wsmz/4Rr/imv3n9keZu/0u75fEA38njoea8TOs5hgYcsdaj2Wuuuuu23mezlGUTxk7vSmt3pppppvufjRRX3N8cv2Kv+FK/Du98f8A/CxvD/if7JNbw/YNK+ec/aJBHvb94dqLnk4POB3yPhmu7A5hh8ZTdXDSuk7bNa/NLucONwNfCTVPERs2r7p6fL0Ciiiu05AooooAKKKKACiiigAooooA/9P8p6KKK/og/BwooooAKKKKACiiigAooooAKKKKACvub9ir45fDv4K/8LH/AOE/vZrT/hJ/D/8AZVh5NvJcZnfzPmfywdqLkZPXngHnHwzRXFmGBp4zDyw1VtJ2230afn2OvA42eErxxFNJtX321Vv1Ciiiu05AooooAKKKKACiiigAooooAKKKKAP/2Q==
".to_string());
    let image = image::load_from_memory(&image_bytes).unwrap();
   let _= image.save("./1.jpg");
    println!("Hello Sunny");
}