use crate::types::*;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TexConfig {
    pub tex_type:     TextureType,
    pub internal_fmt: InternalStorage,
    pub pix_data_fmt: PixelDataFormat,
    pub pix_type_fmt: PixelDataType,
}

impl TexConfig {
    pub fn new(tex_type: TextureType, internal_format: InternalStorage, pix_d_fmt: PixelDataFormat, pix_d_type: PixelDataType) -> TexConfig {
        TexConfig {
            tex_type: tex_type,
            internal_fmt: internal_format,
            pix_data_fmt: pix_d_fmt,
            pix_type_fmt: pix_d_type,
        }
    }

    pub fn validate(&self) -> Result<(), TextureError> {
        if self.pix_data_fmt != PixelDataFormat::RGB {
            match self.pix_type_fmt {
                PixelDataType::UnsignedByte332 | PixelDataType::UnsignedByte233Rev => Err(TextureError::BadTextureConfig),
                PixelDataType::UnsignedShort565 | PixelDataType::UnsignedShort565Rev => Err(TextureError::BadTextureConfig),
                _ => Ok(())
            }?
        }

        if self.pix_data_fmt != PixelDataFormat::RGBA && self.pix_data_fmt != PixelDataFormat::BGRA {
            match self.pix_type_fmt {
                PixelDataType::UnsignedInt1010102 | PixelDataType::UnsignedInt2101010Rev => Err(TextureError::BadTextureConfig),
                PixelDataType::UnsignedInt8888 | PixelDataType::UnsignedInt8888Rev => Err(TextureError::BadTextureConfig),
                PixelDataType::UnsignedShort5551 | PixelDataType::UnsignedShort1555Rev => Err(TextureError::BadTextureConfig),
                PixelDataType::UnsignedShort4444 | PixelDataType::UnsignedShort4444Rev => Err(TextureError::BadTextureConfig),
                _ => Ok(())
            }?
        }

        if self.tex_type != TextureType::Texture2D 
            && self.tex_type != TextureType::ProxyTexture2D 
            && self.tex_type != TextureType::TextureRectangle 
            && self.tex_type != TextureType::ProxyTextureRectangle {
                match self.internal_fmt {
                    InternalStorage::Depth => Err(TextureError::BadTextureConfig),
                    InternalStorage::Depth16 => Err(TextureError::BadTextureConfig),
                    InternalStorage::Depth24 => Err(TextureError::BadTextureConfig),
                    InternalStorage::Depth32 => Err(TextureError::BadTextureConfig),
                    InternalStorage::Float_Depth32 => Err(TextureError::BadTextureConfig),
                    _ => Ok(()),
                }?
        }

        if self.pix_data_fmt != PixelDataFormat::Depth {
            match self.internal_fmt {
                InternalStorage::Depth => Err(TextureError::BadTextureConfig),
                InternalStorage::Depth16 => Err(TextureError::BadTextureConfig),
                InternalStorage::Depth24 => Err(TextureError::BadTextureConfig),
                InternalStorage::Depth32 => Err(TextureError::BadTextureConfig),
                InternalStorage::Float_Depth32 => Err(TextureError::BadTextureConfig),
                _ => Ok(()),
            }?
        }

        if self.pix_data_fmt == PixelDataFormat::Depth {
            match self.internal_fmt {
                InternalStorage::Depth => Ok(()),
                InternalStorage::Depth16 => Ok(()),
                InternalStorage::Depth24 => Ok(()),
                InternalStorage::Depth32 => Ok(()),
                InternalStorage::Float_Depth32 => Ok(()),
                _ => Err(TextureError::BadTextureConfig),
            }?
        }

        Ok(())
    }
}