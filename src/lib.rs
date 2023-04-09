use axum_core::response::{IntoResponse, Response};
use bytes::Bytes;
use http_body::Full;

macro_rules! responses {
    ($($name:ident, $content:literal, $description:literal;)*) => {
        $(
            #[doc = concat!($description, " response")]
            #[doc = ""]
            #[doc = concat!("Will automatically get the `Content-Type: ", $content, "` header.")]
            #[derive(Clone, Copy, Debug)]
            #[must_use]
            pub struct $name<T>(pub T);

            impl<T> IntoResponse for $name<T>
            where
                T: Into<Full<Bytes>>,
            {
                fn into_response(self) -> Response {
                    ([("content-type", $content)], self.0.into()).into_response()
                }
            }

            impl<T> From<T> for $name<T> {
                fn from(inner: T) -> Self {
                    Self(inner)
                }
            }
        )*
    };
}

responses! {
    Aac, "audio/aac", "AAC audio";
    Abw, "application/x-abiword", "AbiWorld document";
    Arc, "application/x-freearc", "Archive document (multiple files embedded)";
    Avif, "image/avif", "AVIF image";
    Avi, "video/x-msvideo", "AVI video";
    Azw, "application/vnd.amazon.ebook", "Amazon Kindle eBook";
    Bmp, "image/bmp", "Windows OS/2 Bitmap Graphics";
    Bz, "application/x-bzip", "BZip archive";
    Bz2, "application/x-bzip2", "BZip2 archive";
    Cda, "application/x-cdf", "CD audio";
    Csh, "application/x-csh", "C-Shell script";
    Css, "text/css", "Cascading Style Sheet";
    Csv, "text/csv", "Comma-separated values";
    Doc, "application/msword", "Microsoft Word";
    Docx, "application/vnd.openxmlformats-officedocument.wordprocessingml.document", "Microsoft Word (OpenXML)";
    Eot, "application/vnd.ms-fontobject", "MS Embedded OpenType fonts";
    Epub, "application/epub+zip", "Electronic publication";
    Gz, "application/gzip", "GZip Compressed Archive";
    Gif, "image/gif", "Graphics Interchange Format";
    Ico, "image/vnd.microsoft.icon", "Icon format";
    Ics, "text/calendar", "iCalendar format";
    Jar, "application/java-archive", "Java Archive";
    Jpg, "image/jpeg", "JPG / JPEG image";
    Js, "text/javascript", "JavaScript";
    Jsonld, "application/ld+json", "JSON-LD format";
    Midi, "audio/midi", "Musical Instrument Digital Interface";
    Mjs, "text/javascript", "JavaScript module";
    Mp3, "audio/mpeg", "MP3 audio";
    Mp4, "video/mp4", "MP4 video";
    Mpeg, "video/mpeg", "MPEG Video";
    Mpkg, "application/vnd.apple.installer+xml", "Apple Installer Package";
    Odp, "application/vnd.oasis.opendocument.presentation", "OpenDocument presentation document";
    Ods, "application/vnd.oasis.opendocument.spreadsheet", "OpenDocument spreadsheet document";
    Odt, "application/vnd.oasis.opendocument.text", "OpenDocument text document";
    Oga, "audio/ogg", "OGG audio";
    Ogv, "video/ogg", "OGG video";
    Ogx, "application/ogg", "OGG";
    Opus, "audio/opus", "Opus audio";
    Otf, "font/otf", "OpenType font";
    Png, "image/png", "PNG image";
    Pdf, "application/pdf", "Portable Document Format";
    Php, "application/x-httpd-php", "PHP";
    Ppt, "application/vnd.ms-powerpoint", "Microsoft PowerPoint";
    Pptx, "application/vnd.openxmlformats-officedocument.presentationml.presentation", "Microsoft PowerPoint (OpenXML)";
    Rar, "application/vnd.rar", "RAR archive";
    Rtf, "application/rtf", "Rich Text Format";
    Sh, "application/x-sh", "Shell script";
    Svg, "image/svg+xml", "Scalable Vector Graphics";
    Tar, "application/x-tar", "Tarball";
    Tiff, "image/tiff", "TIFF / TIF image";
    Toml, "application/toml", "TOML";
    Ts, "video/mp2t", "MPEG transport stream";
    Ttf, "font/ttf", "TrueType Font";
    Vsd, "application/vnd.visio", "Microsoft Visio";
    Wasm, "application/wasm", "WebAssembly";
    Wav, "audio/wav", "Waveform Audio Format";
    Weba, "audio/webm", "WEBM audio";
    Webm, "video/webm", "WEBM video";
    Webp, "image/webp", "WEBP image";
    Woff, "font/woff", "Web Open Font Format";
    Woff2, "font/woff2", "Web Open Font Format";
    Xhtml, "application/xhtml+xml", "Extensible HTML";
    Xls, "application/vnd.ms-excel", "Microsoft Excel";
    Xlsx, "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet", "Microsoft Excel (OpenXML)";
    Xml, "application/xml", "Extensible Markup Language";
    Xul, "application/vnd.mozilla.xul+xml", "XML User Interface Language";
    Zip, "application/zip", "ZIP archive";
}
