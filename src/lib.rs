//! # rsff
//! 
//! `rsff` (scanlation file format) is the core library of an application designed to 
//! facilitate the work of teams translating content such as manga, manhwa, manhua, webtoons, etc.

use balloon::{Balloon, BalloonImage};
use consts::{OUT, TYPES};

use std::ffi::OsStr;
use std::io::{Write, Read};
use std::fs::File;
use std::path::Path;

use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;

use base64::{engine, Engine as _, alphabet};

pub mod balloon;
pub mod consts;

const B64: engine::GeneralPurpose = engine::GeneralPurpose::new(&alphabet::URL_SAFE, engine::general_purpose::NO_PAD);

type XMLConvertResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Debug)]
struct FileDoesNotExists;

/// A document containing all of your translation data.
/// 
/// # Examples
/// 
/// ```
/// use rsff::Document;
/// use rsff::balloon::Balloon;
/// 
/// // Create a default document.
/// let mut d: Document = Document::default();
/// 
/// // Create a default balloon.
/// let mut b: Balloon = Balloon::default();
/// 
/// // Add content to the balloon.
/// b.tl_content.push("This is a translation line.".to_string());
/// 
/// // Add balloon to the document.
/// d.balloons.push(b);
/// ```
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Document {
    /// sff (Scanlation File Format) version. No big changes expected.
    pub METADATA_SCRIPT_VERSION: String,
    /// If you use this library for an app, it may come in handy to indicate your app's version.
    pub METADATA_APP_VERSION: String,
    /// Some other info you want to give/specify.
    pub METADATA_INFO: String,
    /// There is your balloons m8.
    pub balloons: Vec<Balloon>
}

impl Default for Document {
    /// ```notrust
    /// METADATA_SCRIPT_VERSION: String::from("Scanlation Script File v0.2.0"),
    /// METADATA_APP_VERSION: String::new(),
    /// METADATA_INFO: String::from("Num"),
    /// balloons: Vec::new()
    /// ```
    fn default() -> Self {    
        Self {
            METADATA_SCRIPT_VERSION: String::from("Scanlation Script File v0.2.0"),
            METADATA_APP_VERSION: String::new(),
            METADATA_INFO: String::from("Num"),
            balloons: Vec::new()
        }
    }
}

impl Document {
    /// Total character count of all translation content.
    /// *(Spaces included.)*
    pub fn tl_chars(&self) -> usize {
        self.balloons
            .iter()
            .map(|b| {
                b.tl_chars()
            }).sum()
    }

    /// Total character count of all proofread content.
    /// *(Spaces included.)*
    pub fn pr_chars(&self) -> usize {
        self.balloons
            .iter()
            .map(|b| {
                b.pr_chars()
            }).sum()
    }

    /// Total character count of all comments.
    /// *(Spaces included.)*
    pub fn comment_chars(&self) -> usize {
        self.balloons
            .iter()
            .map(|b| {
                b.comments_chars()
            }).sum()
    }

    /// Total line count of the whole document.
    /// Counts pr content lines if balloon has pr content, otherwise counts tl content lines.
    pub fn line_count(&self) -> usize {
        self.balloons
            .iter()
            .map(|b| {
                b.line_count()
            }).sum()
    }

    /// Total balloon count.
    pub fn len(&self) -> usize {
        self.balloons.len()
    }

    /// Generates stringified version of the document.
    /// Use this with caution because of data loss.
    /// 
    /// **IMPORTANT NOTE:** ***Metadata and balloon_img are lost during the creation of the text!!!***
    pub fn to_string(&self) -> String {
        let mut all_text: Vec<String> = Vec::new();

        // No metadata, images etc. Just clean formatted string.
        self.balloons
            .iter()
            .for_each(|b| {
                all_text.push(
                    b.to_string()
                );
            });

        return all_text.join("\n\n");
    }

    /// Generates an xml string of the balloon. No data loss so you can use this whenever you want.
    /// 
    /// **Note:** Raw image data will be converted to a b64 encoded string.
    pub fn to_xml(&self) -> String{
        let mut xml = String::from("<Document><Metadata>");

        // Add script and app related data
        xml.push_str(format!(
            "<Script>{}</Script>\
            <App>{}</App>\
            <Info>{}</Info>",
            self.METADATA_SCRIPT_VERSION,
            self.METADATA_APP_VERSION,
            self.METADATA_INFO
        ).as_str());

        // Add other data
        xml.push_str(format!(
            "<TLLength>{}</TLLength>\
            <PRLength>{}</PRLength>\
            <CMLength>{}</CMLength>\
            <BalloonCount>{}</BalloonCount>\
            <LineCount>{}</LineCount>",
            self.tl_chars(),
            self.pr_chars(),
            self.comment_chars(),
            self.balloons.len(),
            self.line_count()
        ).as_str());

        xml.push_str("</Metadata>");
        xml.push_str("<Balloons>");

        // Add all balloons
        self.balloons
            .iter()
            .for_each(|b| {
                xml.push_str(b.to_xml().as_str());
            });
        
        xml.push_str("</Balloons>");
        xml.push_str("</Document>");
        
        return xml;
    }

    // Save as a raw xml file.
    fn save_raw(&self, fp: &str) {
        let mut file = File::create(
            format!("{fp}.sffx")
        ).unwrap();
        file.write(self.to_xml().as_bytes()).unwrap();
    }

    // Save as a compressed xml file.
    fn save_zlib(&self, fp: &str) {
        let mut f = File::create(format!("{fp}.sffz")).unwrap();
        let mut enc = ZlibEncoder::new(Vec::new(), Compression::best());
        enc.write_all(self.to_xml().as_bytes()).unwrap();
        let encoded = enc.finish().unwrap();
        f.write(&encoded).unwrap();
    }

    /// Save your document as raw xml, compressed xml or .txt file.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rsff::Document;
    /// use rsff::consts::OUT;
    /// 
    /// let d = Document::default();
    /// 
    /// // Save as raw xml:
    /// d.save(OUT::RAW, "raw_xml");
    /// 
    /// // Save as ZLIB compressed xml:
    /// d.save(OUT::ZLIB, "compressed_xml");
    /// 
    /// // Save as raw text:
    /// d.save(OUT::TXT, "raw_text");
    /// ```
    pub fn save(&self, out_type: OUT, fp: &str) {
        match out_type {
            OUT::RAW => self.save_raw(fp),
            OUT::TXT => {
                let f_name = format!("{}.txt", fp);
                let mut f = File::create(f_name).unwrap();
                f.write(self.to_string().as_bytes()).unwrap();
            },
            OUT::ZLIB => self.save_zlib(fp)
        }
    }

    // Generate text of the whole document.
    fn file_to_string(&self, p: &Path) -> String {
        let mut s = String::new();
        let mut f = File::open(p).unwrap();
        f.read_to_string(&mut s).unwrap();

        return s;
    }

    // Open a file and return it's byte content.
    fn file_to_bytes(&self, p: &Path) -> Vec<u8> {
        let mut buff: Vec<u8> = Vec::new();
        let mut f = File::open(p).unwrap();
        f.read_to_end(&mut buff).unwrap();

        return buff;
    }

    // Generate a document from xml string.
    pub fn xml_to_doc(&mut self, xml: String) -> XMLConvertResult<Document> {
        // Create an empty document
        let mut d = Document::default();

        // Parse xml string
        let tree = roxmltree::Document::parse(&xml)?;

        // Find metadata tag
        let md = tree.descendants().find(|d| {d.tag_name().name() == "Metadata"}).unwrap();

        // Register file's metadata as document's metadata
        // Note: Some other metadata like tl_chars / tl_content are dynamically 
        // thus no need to register them.
        d.METADATA_SCRIPT_VERSION = md.children().find(|c| {c.tag_name().name() == "Script"}).unwrap().text().unwrap_or("").to_string();
        d.METADATA_APP_VERSION = md.children().find(|c| {c.tag_name().name() == "App"}).unwrap().text().unwrap_or("").to_string();
        d.METADATA_INFO = md.children().find(|c| {c.tag_name().name() == "Info"}).unwrap().text().unwrap_or("").to_string();

        // Find Balloons tag
        let bs = tree.descendants().find(|c| {c.tag_name().name() == "Balloons"}).unwrap();

        // Iterate over all xml balloons and generate Balloon struct, then add those structs to document
        for c in bs.children() {
            let mut b = Balloon {
                btype: match c.attribute("type").unwrap() {
                    "Dialogue" => TYPES::DIALOGUE,
                    "Square" => TYPES::SQUARE,
                    "ST" => TYPES::ST,
                    "OT" => TYPES::OT,
                    "Thinking" => TYPES::THINKING,
                    _ => TYPES::DIALOGUE
                },
                ..Default::default()
            };

            let tls = c.children().filter(|c| {c.tag_name().name() == "TL"});
            let prs = c.children().filter(|c| {c.tag_name().name() == "PR"});
            let comments = c.children().filter(|c| {c.tag_name().name() == "Comment"});
            let img = c.children().find(|c| {c.tag_name().name() == "img"});

            for tl in tls {
                let content = match tl.text() {
                    Some(t) => t.to_string(),
                    None => String::new()
                };
                b.tl_content.push(content);
            }

            for pr in prs {
                let content = match pr.text() {
                    Some(t) => t.to_string(),
                    None => String::new()
                };
                b.pr_content.push(content);
            }

            for comment in comments {
                let content = match comment.text() {
                    Some(t) => t.to_string(),
                    None => String::new()
                };
                b.comments.push(content);
            }

            if img.is_some() {
                let i = BalloonImage {
                    img_type: img.unwrap().attribute("type").unwrap().to_string(),
                    img_data: B64.decode(img.unwrap().text().unwrap())?
                };
                b.balloon_img = Some(i);
            } else {
                b.balloon_img = None;
            }

            d.balloons.push(b);
        }

        return Ok(d);
    }

    fn decide_b_type_from_txt_line_headers(&self, ln: &str) -> TYPES {
        let s = &ln[0..2];

        match s {
            "()" => TYPES::DIALOGUE,
            "OT" => TYPES::OT,
            "[]" => TYPES::SQUARE,
            "ST" => TYPES::ST,
            "{}" => TYPES::THINKING,
            _ => TYPES::DIALOGUE
        }
    }

    // Generate a document from lossy text.
    // Why did i write this?
    // This is probably most unnecessary code ib this crate.
    fn txt_to_doc(&self, txt: String) -> XMLConvertResult<Document> {
        let mut d = Document::default();
        let mut texts: Vec<String> = Vec::with_capacity(10);

        let splitted = txt.split("\n").filter(|s| {!s.is_empty()}).collect::<Vec<&str>>();
        let mut is_previous_double_slash: bool = false;

        for i in 0..splitted.len() {
            if splitted[i].contains("//") {continue;}

            let current = splitted[i];

            let mut b = Balloon::default();
            b.btype = self.decide_b_type_from_txt_line_headers(current);
            
            let next = splitted.get(i+1).unwrap_or(&"");

            if !next.contains("//") {
                if is_previous_double_slash {
                    texts.push(current[4..current.len()].trim().to_string());
                    b.tl_content = texts.clone();
                    d.balloons.push(b);
                    is_previous_double_slash = false;
                    continue;
                } else {
                    b.tl_content.push(current[4..current.len()].trim().to_string());
                    d.balloons.push(b);
                    is_previous_double_slash = false;
                    continue;
                }
            } else {
                texts.push(current[4..current.len()].trim().to_string());
                is_previous_double_slash = true;
            }         
        }

        return Ok(d);
    }

    /// Open a supported sffx, sffz or txt file and generate a document.
    /// 
    /// `fp`: full path for the file.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rsff::Document;
    /// 
    /// let mut d: Document = Document::default().open("test.sffx").unwrap().unwrap();
    /// ```
    /// 
    /// **Note:** I messed up this absolutely shitty method and will change it in the future definitely.
    pub fn open(&mut self, fp: &str) -> Result<XMLConvertResult<Document>, &str> {
        let p = Path::new(fp);

        if !p.exists() {return Err("File does not exists!")}

        match p.extension() {
            None => {return Err("No file ext!");},
            Some(e) => {
                if e == OsStr::new("txt") {
                    let text = self.file_to_string(p);
                    return Ok(self.txt_to_doc(text));
                } else if e == OsStr::new("sffx") {
                    let xml = self.file_to_string(p);
                    return Ok(self.xml_to_doc(xml));
                } else if e == OsStr::new("sffz") {
                    let compressed = self.file_to_bytes(p);
                    let mut xml = String::new();
                    let mut decoder = ZlibDecoder::new(&*compressed);
                    decoder.read_to_string(&mut xml).unwrap();
                    return Ok(self.xml_to_doc(xml));
                } else {
                    return Err("Unsupported file type!");
                }
            }
        }
    }
}

#[cfg(test)]
mod document_related {
    use std::io::Read;
    use std::fs::File;
    use flate2::read::ZlibDecoder;

    use crate::Document;
    use crate::balloon::Balloon;
    use crate::consts::{TYPES, OUT};

    #[test]
    fn document_tl_chars() {
        let mut d = Document::default();
        let mut b1 = Balloon::default();
        let mut b2 = Balloon::default();

        b1.tl_content.push(String::from("num"));
        b2.tl_content.push(String::from("num"));
        b2.tl_content.push(String::from("namnam"));

        d.balloons.push(b1);
        d.balloons.push(b2);

        assert_eq!(
            d.tl_chars(),
            12
        )
    }

    #[test]
    fn document_pr_chars() {
        let mut d = Document::default();
        let mut b1 = Balloon::default();
        let mut b2 = Balloon::default();

        b1.pr_content.push(String::from("num"));
        b2.pr_content.push(String::from("num"));
        b2.pr_content.push(String::from("namnam"));

        d.balloons.push(b1);
        d.balloons.push(b2);

        assert_eq!(
            d.pr_chars(),
            12
        )
    }

    #[test]
    fn document_comment_chars() {
        let mut d = Document::default();
        let mut b1 = Balloon::default();
        let mut b2 = Balloon::default();

        b1.comments.push(String::from("num"));
        b2.comments.push(String::from("num"));
        b2.comments.push(String::from("namnam"));

        d.balloons.push(b1);
        d.balloons.push(b2);

        assert_eq!(
            d.comment_chars(),
            12
        )
    }

    #[test]
    fn document_line_count() {
        let mut d = Document::default();
        let mut b1 = Balloon::default();
        let mut b2 = Balloon::default();

        b1.tl_content.push(String::from("num"));
        b2.tl_content.push(String::from("num"));
        b2.pr_content.push(String::from("namnam"));

        d.balloons.push(b1);
        d.balloons.push(b2);

        assert_eq!(
            d.line_count(),
            2
        )
    }

    #[test]
    fn document_length() {
        let mut d = Document::default();
        let b1 = Balloon::default();
        let b2 = Balloon::default();

        d.balloons.push(b1);
        d.balloons.push(b2);

        assert_eq!(
            d.len(),
            2
        )
    }

    #[test]
    fn document_to_string() {
        let mut d = Document::default();
        let mut b1 = Balloon::default();
        let mut b2 = Balloon::default();

        b1.tl_content.push(String::from("num"));
        b1.tl_content.push(String::from("nam"));
        b1.pr_content.push(String::from("numnam"));
        b1.btype = TYPES::OT;

        b2.tl_content.push(String::from("num"));

        d.balloons.push(b1);
        d.balloons.push(b2);

        d.save(OUT::TXT, "test");

        let mut s = String::new();
        let mut f = File::open("test.txt").unwrap();
        f.read_to_string(&mut s).unwrap();

        assert_eq!(
            s,
            String::from("OT: numnam\n\n(): num")
        )
    }

    #[test]
    fn document_to_xml() {
        let mut d = Document::default();
        let mut b1 = Balloon::default();
        let mut b2 = Balloon::default();

        b1.tl_content.push(String::from("num"));
        b1.tl_content.push(String::from("nam"));
        b1.pr_content.push(String::from("numnam"));
        b1.btype = TYPES::OT;

        b2.tl_content.push(String::from("num"));

        d.balloons.push(b1);
        d.balloons.push(b2);

        d.save(OUT::RAW, "test");

        let num = String::from(r#"<Document><Metadata><Script>Scanlation Script File v0.2.0</Script><App></App><Info>Num</Info><TLLength>9</TLLength><PRLength>6</PRLength><CMLength>0</CMLength><BalloonCount>2</BalloonCount><LineCount>2</LineCount></Metadata><Balloons><Balloon type="OT"><TL>num</TL><TL>nam</TL><PR>numnam</PR></Balloon><Balloon type="Dialogue"><TL>num</TL></Balloon></Balloons></Document>"#);
        let mut created = String::new();
        let mut f = File::open("test.sffx").unwrap();
        f.read_to_string(&mut created).unwrap();

        assert_eq!(num, created)
    }

    #[test]
    fn document_to_compressed() {
        let mut d = Document::default();
        let mut b1 = Balloon::default();
        let mut b2 = Balloon::default();

        b1.tl_content.push(String::from("num"));
        b1.tl_content.push(String::from("nam"));
        b1.pr_content.push(String::from("numnam"));
        b1.btype = TYPES::OT;

        b2.tl_content.push(String::from("num"));

        d.balloons.push(b1);
        d.balloons.push(b2);

        d.save(OUT::ZLIB, "test");

        let num = String::from(r#"<Document><Metadata><Script>Scanlation Script File v0.2.0</Script><App></App><Info>Num</Info><TLLength>9</TLLength><PRLength>6</PRLength><CMLength>0</CMLength><BalloonCount>2</BalloonCount><LineCount>2</LineCount></Metadata><Balloons><Balloon type="OT"><TL>num</TL><TL>nam</TL><PR>numnam</PR></Balloon><Balloon type="Dialogue"><TL>num</TL></Balloon></Balloons></Document>"#);
        let mut created = String::new();
        let mut f = File::open("test.sffz").unwrap();
        let mut encoded = Vec::new();
        f.read_to_end(&mut encoded).unwrap();
        let mut decoder = ZlibDecoder::new(&*encoded);
        decoder.read_to_string(&mut created).unwrap();

        assert_eq!(num, created)
    }

    #[test]
    fn document_open_txt() {
        let d = Document::default().open("test.txt").unwrap().unwrap();

        assert_eq!(d.line_count(), 2);
        assert_eq!(d.balloons.len(), 2);
        assert_eq!(d.balloons[0].btype, TYPES::OT);
        assert_eq!(d.balloons[0].tl_content[0], "numnam");
        assert_eq!(d.balloons[1].btype, TYPES::DIALOGUE);
        assert_eq!(d.balloons[1].tl_content[0], "num");
    }

    #[test]
    fn document_open_sffx() {
        let d = Document::default().open("test.sffx").unwrap().unwrap();
        let case = r#"<Document><Metadata><Script>Scanlation Script File v0.2.0</Script><App></App><Info>Num</Info><TLLength>9</TLLength><PRLength>6</PRLength><CMLength>0</CMLength><BalloonCount>2</BalloonCount><LineCount>2</LineCount></Metadata><Balloons><Balloon type="OT"><TL>num</TL><TL>nam</TL><PR>numnam</PR></Balloon><Balloon type="Dialogue"><TL>num</TL></Balloon></Balloons></Document>"#;
        assert_eq!(
            d.to_xml(),
            case
        );
    }

    #[test]
    fn document_open_sffz() {
        let d = Document::default().open("test.sffz").unwrap().unwrap();
        let case = String::from(r#"<Document><Metadata><Script>Scanlation Script File v0.2.0</Script><App></App><Info>Num</Info><TLLength>9</TLLength><PRLength>6</PRLength><CMLength>0</CMLength><BalloonCount>2</BalloonCount><LineCount>2</LineCount></Metadata><Balloons><Balloon type="OT"><TL>num</TL><TL>nam</TL><PR>numnam</PR></Balloon><Balloon type="Dialogue"><TL>num</TL></Balloon></Balloons></Document>"#);
        assert_eq!(
            d.to_xml(),
            case
        );
    }

    #[test]
    fn document_unsupported_file_ext() {
        let mut d = Document::default();
        let r = d.open("test.test");
        if r.is_err() {
            assert!(true)
        }
    }
}