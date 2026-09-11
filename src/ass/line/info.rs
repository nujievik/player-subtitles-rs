#[derive(Debug, PartialEq)]
pub enum ScriptInfo<'a> {
    Title(Title<'a>),
    OriginalScript(OriginalScript<'a>),
    OriginalTranslation(OriginalTranslation<'a>),
    OriginalEditing(OriginalEditing<'a>),
    OriginalTiming(OriginalTiming<'a>),
    SynchPoint(SynchPoint<'a>),
    ScriptUpdatedBy(ScriptUpdatedBy<'a>),
    UpdateDetails(UpdateDetails<'a>),
    ScriptType(ScriptType<'a>),
    Collisions(Collisions<'a>),
    PlayResY(PlayResY<'a>),
    PlayResX(PlayResX<'a>),
    PlayDepth(PlayDepth<'a>),
    Timer(Timer<'a>),
    WrapStyle(WrapStyle<'a>),
}

impl<'a> ScriptInfo<'a> {
    pub(crate) fn get_new(bytes: &'a [u8]) -> Option<Self> {
        let mut it = bytes.splitn(2, |b| matches!(b, b':'));
        let (left, _) = (it.next()?, it.next()?);

        let x = match left {
            b"Title" => ScriptInfo::Title(Title { bytes }),
            b"Original Script" => ScriptInfo::OriginalScript(OriginalScript { bytes }),
            b"Original Translation" => {
                ScriptInfo::OriginalTranslation(OriginalTranslation { bytes })
            }
            b"Original Editing" => ScriptInfo::OriginalEditing(OriginalEditing { bytes }),
            b"Original Timing" => ScriptInfo::OriginalTiming(OriginalTiming { bytes }),
            b"Synch Point" => ScriptInfo::SynchPoint(SynchPoint { bytes }),
            b"Script Updated By" => ScriptInfo::ScriptUpdatedBy(ScriptUpdatedBy { bytes }),
            b"Update Details" => ScriptInfo::UpdateDetails(UpdateDetails { bytes }),
            b"ScriptType" => ScriptInfo::ScriptType(ScriptType { bytes }),
            b"Collisions" => ScriptInfo::Collisions(Collisions { bytes }),
            b"PlayResY" => ScriptInfo::PlayResY(PlayResY { bytes }),
            b"PlayResX" => ScriptInfo::PlayResX(PlayResX { bytes }),
            b"PlayDepth" => ScriptInfo::PlayDepth(PlayDepth { bytes }),
            b"Timer" => ScriptInfo::Timer(Timer { bytes }),
            b"WrapStyle" => ScriptInfo::WrapStyle(WrapStyle { bytes }),
            _ => return None,
        };
        Some(x)
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Title(x) => x.as_bytes(),
            Self::OriginalScript(x) => x.as_bytes(),
            Self::OriginalTranslation(x) => x.as_bytes(),
            Self::OriginalEditing(x) => x.as_bytes(),
            Self::OriginalTiming(x) => x.as_bytes(),
            Self::SynchPoint(x) => x.as_bytes(),
            Self::ScriptUpdatedBy(x) => x.as_bytes(),
            Self::UpdateDetails(x) => x.as_bytes(),
            Self::ScriptType(x) => x.as_bytes(),
            Self::Collisions(x) => x.as_bytes(),
            Self::PlayResY(x) => x.as_bytes(),
            Self::PlayResX(x) => x.as_bytes(),
            Self::PlayDepth(x) => x.as_bytes(),
            Self::Timer(x) => x.as_bytes(),
            Self::WrapStyle(x) => x.as_bytes(),
        }
    }
}

bytes_field_struct!(Title);
bytes_field_struct!(OriginalScript);
bytes_field_struct!(OriginalTranslation);
bytes_field_struct!(OriginalEditing);
bytes_field_struct!(OriginalTiming);
bytes_field_struct!(SynchPoint);
bytes_field_struct!(ScriptUpdatedBy);
bytes_field_struct!(UpdateDetails);
bytes_field_struct!(ScriptType);
bytes_field_struct!(Collisions);
bytes_field_struct!(PlayResY);
bytes_field_struct!(PlayResX);
bytes_field_struct!(PlayDepth);
bytes_field_struct!(Timer);
bytes_field_struct!(WrapStyle);
