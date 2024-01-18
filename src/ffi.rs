use crate::DXGI_FORMAT;

#[allow(non_snake_case)]
#[link(name = "directxtex-ffi")]
extern "C" {
    pub(crate) fn DirectXTexFFI_IsValid(fmt: DXGI_FORMAT) -> bool;
    pub(crate) fn DirectXTexFFI_IsCompressed(fmt: DXGI_FORMAT) -> bool;
    pub(crate) fn DirectXTexFFI_IsPacked(fmt: DXGI_FORMAT) -> bool;
    pub(crate) fn DirectXTexFFI_IsVideo(fmt: DXGI_FORMAT) -> bool;
    pub(crate) fn DirectXTexFFI_IsPlanar(fmt: DXGI_FORMAT) -> bool;
    pub(crate) fn DirectXTexFFI_IsPalettized(fmt: DXGI_FORMAT) -> bool;
    pub(crate) fn DirectXTexFFI_IsDepthStencil(fmt: DXGI_FORMAT) -> bool;
    pub(crate) fn DirectXTexFFI_IsSRGB(fmt: DXGI_FORMAT) -> bool;
    pub(crate) fn DirectXTexFFI_IsBGR(fmt: DXGI_FORMAT) -> bool;
    pub(crate) fn DirectXTexFFI_IsTypeless(fmt: DXGI_FORMAT, partialTypeless: bool) -> bool;
}
