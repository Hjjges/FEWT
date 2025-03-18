#[derive(Debug, Clone, PartialEq, Eq, strum_macros::Display)]
pub enum FileType {
    // Documents and Text
    #[strum(to_string = "Text File")]
    Txt,
    #[strum(to_string = "Markdown Document")]
    Markdown,
    #[strum(to_string = "PDF Document")]
    Pdf,
    #[strum(to_string = "Word Document")]
    Word,
    #[strum(to_string = "Excel Spreadsheet")]
    Excel,
    #[strum(to_string = "PowerPoint Presentation")]
    PowerPoint,
    #[strum(to_string = "CSV Spreadsheet")]
    Csv,
    #[strum(to_string = "Rich Text Document")]
    Rtf,
    #[strum(to_string = "OpenDocument Text")]
    Odt,
    #[strum(to_string = "OpenDocument Spreadsheet")]
    Ods,
    #[strum(to_string = "OpenDocument Presentation")]
    Odp,
    #[strum(to_string = "LaTeX Document")]
    Latex,
    #[strum(to_string = "EPUB eBook")]
    Epub,
    #[strum(to_string = "Mobipocket eBook")]
    Mobi,

    // Programming and Config
    #[strum(to_string = "Rust Source")]
    Rust,
    #[strum(to_string = "TOML Config")]
    Toml,
    #[strum(to_string = "JSON Data")]
    Json,
    #[strum(to_string = "YAML Data")]
    Yaml,
    #[strum(to_string = "XML Data")]
    Xml,
    #[strum(to_string = "HTML Document")]
    Html,
    #[strum(to_string = "CSS Stylesheet")]
    Css,
    #[strum(to_string = "JavaScript Source")]
    JavaScript,
    #[strum(to_string = "TypeScript Source")]
    TypeScript,
    #[strum(to_string = "Python Source")]
    Python,
    #[strum(to_string = "Java Source")]
    Java,
    #[strum(to_string = "C Source")]
    C,
    #[strum(to_string = "C++ Source")]
    Cpp,
    #[strum(to_string = "C/C++ Header")]
    Header,
    #[strum(to_string = "C# Source")]
    CSharp,
    #[strum(to_string = "Go Source")]
    Go,
    #[strum(to_string = "Ruby Source")]
    Ruby,
    #[strum(to_string = "PHP Source")]
    Php,
    #[strum(to_string = "Shell Script")]
    Shell,
    #[strum(to_string = "Swift Source")]
    Swift,
    #[strum(to_string = "SQL Script")]
    Sql,
    #[strum(to_string = "INI Config")]
    Ini,
    #[strum(to_string = "Config File")]
    Config,

    // Images
    #[strum(to_string = "JPEG Image")]
    Jpeg,
    #[strum(to_string = "PNG Image")]
    Png,
    #[strum(to_string = "GIF Image")]
    Gif,
    #[strum(to_string = "SVG Vector")]
    Svg,
    #[strum(to_string = "Bitmap Image")]
    Bmp,
    #[strum(to_string = "TIFF Image")]
    Tiff,
    #[strum(to_string = "WebP Image")]
    Webp,
    #[strum(to_string = "Icon File")]
    Ico,
    #[strum(to_string = "Targa Image")]
    Tga,
    #[strum(to_string = "Raw Image")]
    Raw,
    #[strum(to_string = "Photoshop Document")]
    Psd,
    #[strum(to_string = "Illustrator File")]
    Ai,
    #[strum(to_string = "Camera RAW")]
    CameraRaw,
    #[strum(to_string = "HEIF/HEIC Image")]
    Heif,
    #[strum(to_string = "ASTC Texture")]
    Astc,
    #[strum(to_string = "DirectDraw Surface")]
    Dds,
    #[strum(to_string = "OpenEXR Image")]
    Exr,

    // Audio
    #[strum(to_string = "MP3 Audio")]
    Mp3,
    #[strum(to_string = "WAV Audio")]
    Wav,
    #[strum(to_string = "OGG Audio")]
    Ogg,
    #[strum(to_string = "FLAC Audio")]
    Flac,
    #[strum(to_string = "AAC Audio")]
    Aac,
    #[strum(to_string = "Windows Media Audio")]
    Wma,
    #[strum(to_string = "AIFF Audio")]
    Aiff,
    #[strum(to_string = "MPEG-4 Audio")]
    M4a,
    #[strum(to_string = "MIDI Audio")]
    Midi,

    // Video
    #[strum(to_string = "MP4 Video")]
    Mp4,
    #[strum(to_string = "AVI Video")]
    Avi,
    #[strum(to_string = "Matroska Video")]
    Mkv,
    #[strum(to_string = "QuickTime Video")]
    Mov,
    #[strum(to_string = "Windows Media Video")]
    Wmv,
    #[strum(to_string = "Flash Video")]
    Flv,
    #[strum(to_string = "WebM Video")]
    Webm,
    #[strum(to_string = "MPEG-4 Video")]
    M4v,
    #[strum(to_string = "3GPP Video")]
    Mobile3gp,
    #[strum(to_string = "MPEG Video")]
    Mpeg,

    // Archives and Disk Images
    #[strum(to_string = "ZIP Archive")]
    Zip,
    #[strum(to_string = "TAR Archive")]
    Tar,
    #[strum(to_string = "GZIP Compressed")]
    Gzip,
    #[strum(to_string = "Compressed TAR")]
    Tgz,
    #[strum(to_string = "7-Zip Archive")]
    SevenZip,
    #[strum(to_string = "RAR Archive")]
    Rar,
    #[strum(to_string = "BZIP2 Compressed")]
    Bzip2,
    #[strum(to_string = "XZ Compressed")]
    Xz,
    #[strum(to_string = "macOS Disk Image")]
    Dmg,
    #[strum(to_string = "ISO Disk Image")]
    Iso,
    #[strum(to_string = "Disk Image")]
    Img,
    #[strum(to_string = "Virtual Hard Disk")]
    Vhd,
    #[strum(to_string = "VMware Disk")]
    Vmdk,

    // Fonts
    #[strum(to_string = "TrueType Font")]
    Ttf,
    #[strum(to_string = "OpenType Font")]
    Otf,
    #[strum(to_string = "Web Font")]
    Woff,
    #[strum(to_string = "Embedded OpenType Font")]
    Eot,

    // Executables and Binaries
    #[strum(to_string = "Windows Executable")]
    Exe,
    #[strum(to_string = "Dynamic Link Library")]
    Dll,
    #[strum(to_string = "Shared Object Library")]
    So,
    #[strum(to_string = "macOS Application")]
    App,
    #[strum(to_string = "Android Package")]
    Apk,
    #[strum(to_string = "Debian Package")]
    Deb,
    #[strum(to_string = "RPM Package")]
    Rpm,
    #[strum(to_string = "Windows Installer")]
    Msi,

    // Special cases
    #[strum(to_string = "File")] // no extension
    None,
    #[strum(to_string = "Unknown")]
    Unknown(String),
}

pub fn detect_extension(extension: String) -> FileType {
    match extension.to_lowercase().as_str() {
        // Documents and Text
        "txt" => FileType::Txt,
        "md" | "markdown" => FileType::Markdown,
        "pdf" => FileType::Pdf,
        "doc" | "docx" => FileType::Word,
        "xls" | "xlsx" => FileType::Excel,
        "ppt" | "pptx" => FileType::PowerPoint,
        "csv" => FileType::Csv,
        "rtf" => FileType::Rtf,
        "odt" => FileType::Odt,
        "ods" => FileType::Ods,
        "odp" => FileType::Odp,
        "tex" => FileType::Latex,
        "epub" => FileType::Epub,
        "mobi" => FileType::Mobi,

        // Programming and Config
        "rs" => FileType::Rust,
        "toml" => FileType::Toml,
        "json" => FileType::Json,
        "yaml" | "yml" => FileType::Yaml,
        "xml" => FileType::Xml,
        "html" | "htm" => FileType::Html,
        "css" => FileType::Css,
        "js" => FileType::JavaScript,
        "ts" => FileType::TypeScript,
        "py" => FileType::Python,
        "java" => FileType::Java,
        "c" => FileType::C,
        "cpp" | "cc" | "cxx" => FileType::Cpp,
        "h" | "hpp" => FileType::Header,
        "cs" => FileType::CSharp,
        "go" => FileType::Go,
        "rb" => FileType::Ruby,
        "php" => FileType::Php,
        "sh" | "bash" => FileType::Shell,
        "swift" => FileType::Swift,
        "sql" => FileType::Sql,
        "ini" => FileType::Ini,
        "conf" => FileType::Config,

        // Images
        "jpg" | "jpeg" => FileType::Jpeg,
        "png" => FileType::Png,
        "gif" => FileType::Gif,
        "svg" => FileType::Svg,
        "bmp" => FileType::Bmp,
        "tiff" | "tif" => FileType::Tiff,
        "webp" => FileType::Webp,
        "ico" => FileType::Ico,
        "tga" => FileType::Tga,
        "raw" => FileType::Raw,
        "psd" => FileType::Psd,
        "ai" => FileType::Ai,
        "cr2" | "nef" | "arw" => FileType::CameraRaw,
        "heif" | "heic" => FileType::Heif,
        "astc" => FileType::Astc,
        "dds" => FileType::Dds,
        "exr" => FileType::Exr,

        // Audio
        "mp3" => FileType::Mp3,
        "wav" => FileType::Wav,
        "ogg" => FileType::Ogg,
        "flac" => FileType::Flac,
        "aac" => FileType::Aac,
        "wma" => FileType::Wma,
        "aiff" | "aif" => FileType::Aiff,
        "m4a" => FileType::M4a,
        "mid" | "midi" => FileType::Midi,

        // Video
        "mp4" => FileType::Mp4,
        "avi" => FileType::Avi,
        "mkv" => FileType::Mkv,
        "mov" => FileType::Mov,
        "wmv" => FileType::Wmv,
        "flv" => FileType::Flv,
        "webm" => FileType::Webm,
        "m4v" => FileType::M4v,
        "3gp" => FileType::Mobile3gp,
        "mpg" | "mpeg" => FileType::Mpeg,

        // Archives and Disk Images
        "zip" => FileType::Zip,
        "tar" => FileType::Tar,
        "gz" | "gzip" => FileType::Gzip,
        "tgz" => FileType::Tgz,
        "7z" => FileType::SevenZip,
        "rar" => FileType::Rar,
        "bz2" => FileType::Bzip2,
        "xz" => FileType::Xz,
        "dmg" => FileType::Dmg,
        "iso" => FileType::Iso,
        "img" => FileType::Img,
        "vhd" | "vhdx" => FileType::Vhd,
        "vmdk" => FileType::Vmdk,

        // Fonts
        "ttf" => FileType::Ttf,
        "otf" => FileType::Otf,
        "woff" | "woff2" => FileType::Woff,
        "eot" => FileType::Eot,

        // Executables and Binaries
        "exe" => FileType::Exe,
        "dll" => FileType::Dll,
        "so" => FileType::So,
        "app" => FileType::App,
        "apk" => FileType::Apk,
        "deb" => FileType::Deb,
        "rpm" => FileType::Rpm,
        "msi" => FileType::Msi,

        // Special cases
        "" => FileType::None,

        // Default case for unknown extensions
        ext => FileType::Unknown(ext.to_string()),
    }
}