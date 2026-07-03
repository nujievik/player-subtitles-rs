pub enum ScriptInfo<'a> {
    Title(Title<'a>),
    OriginalScript,
    OriginalTranslation,
    OriginalEditing,
    OriginalTiming,
    SynchPoint,
    ScriptUpdatedBy,
    UpdateDetails,
    ScriptType(ScriptType<'a>),
    Collisions,
    PlayResY,
    PlayResX,
    PlayDepth,
    Timer,
    WrapStyle(WrapStyle<'a>),
}

impl<'a> ScriptInfo<'a> {
    pub(crate) fn get_new(bytes: &'a [u8]) -> Option<Self> {
        let mut it = bytes.splitn(2, |b| matches!(b, b':'));
        let (left, _) = (it.next()?, it.next()?);

        let x = match left {
            b"Title" => ScriptInfo::Title(Title { bytes }),
            b"Original Script" => ScriptInfo::OriginalScript,
            b"Original Translation" => ScriptInfo::OriginalTranslation,
            b"Original Editing" => ScriptInfo::OriginalEditing,
            b"Original Timing" => ScriptInfo::OriginalTiming,
            b"Synch Point" => ScriptInfo::SynchPoint,
            b"Script Updated By" => ScriptInfo::ScriptUpdatedBy,
            b"Update Details" => ScriptInfo::UpdateDetails,
            b"ScriptType" => ScriptInfo::ScriptType(ScriptType { bytes }),
            b"Collisions" => ScriptInfo::Collisions,
            b"PlayResY" => ScriptInfo::PlayResY,
            b"PlayResX" => ScriptInfo::PlayResX,
            b"PlayDepth" => ScriptInfo::PlayDepth,
            b"Timer" => ScriptInfo::Timer,
            b"WrapStyle" => ScriptInfo::WrapStyle(WrapStyle { bytes }),
            _ => return None,
        };
        Some(x)
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Title(Title { bytes }) => bytes,
            Self::ScriptType(ScriptType { bytes }) => bytes,
            Self::WrapStyle(WrapStyle { bytes }) => bytes,
            _ => todo!(),
        }
    }
}

pub struct Title<'a> {
    pub(crate) bytes: &'a [u8],
}

pub struct ScriptType<'a> {
    pub(crate) bytes: &'a [u8],
}

pub struct WrapStyle<'a> {
    pub(crate) bytes: &'a [u8],
}
