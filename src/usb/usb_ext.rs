use stm32l151::USB;

pub trait UsbExt {
    fn toggle_ep0_tx_out(&self);
    fn toggle_ep0_tx_stall(&self);
    fn toggle_ep0_out(&self);
    fn toggle_ep0_0(&self);
    fn toggle_ep0(&self, mask: u32, val: u32, flags: u32);

    fn set_ep1_tx_status_valid_dtog(&self);
}

//(USB_EP_CTR_RX|USB_EP_SETUP|USB_EP_T_FIELD|USB_EP_KIND|USB_EP_CTR_TX|USB_EPADDR_FIELD);
const USB_EPREG_MASK: u32 = (1 << 15 | 1 << 11 | 1 << 10 | 1 << 9 | 1 << 8 | 0xf);

const USB_EPTX_STAT: u32 = 0x30;
const USB_EPTX_DTOGMASK: u32 = (USB_EPTX_STAT | USB_EPREG_MASK);

const USB_EP_CTR_RX: u32 = 0x8000;
const USB_EP_CTR_TX: u32 = 0x8000_0000;

const EP_MASK: u32 = 0x0F0F;
const EP_TX_MASK: u32 = 0x0030;
const EP_RX_MASK: u32 = 0x3000;
const EP_TX_RX_MASK: u32 = (EP_TX_MASK | EP_RX_MASK);

const EP_TX_VALID: u32 = 0x0030;
const EP_RX_VALID: u32 = 0x3000;
const EP_TX_RX_VALID: u32 = (EP_TX_VALID | EP_RX_VALID);

const EP_TX_STALL: u32 = 0x0010;
const EP_STATUS_OUT: u32 = 0x0100;

impl UsbExt for USB {
    fn toggle_ep0_tx_stall(&self) {
        self.toggle_ep0(EP_TX_RX_MASK, EP_RX_VALID | EP_TX_STALL, 0)
    }

    fn toggle_ep0_tx_out(&self) {
        self.toggle_ep0(EP_TX_MASK, EP_TX_VALID, EP_STATUS_OUT)
    }

    fn toggle_ep0_out(&self) {
        self.toggle_ep0(EP_TX_RX_MASK, EP_TX_RX_VALID, EP_STATUS_OUT)
    }

    fn toggle_ep0_0(&self) {
        self.toggle_ep0(EP_TX_RX_MASK, EP_TX_RX_VALID, 0)
    }

    fn toggle_ep0(&self, mask: u32, val: u32, flags: u32) {
        self.usb_ep0r
            .modify(|r, w| unsafe { w.bits(((r.bits() & (EP_MASK | mask)) ^ val) | flags) })
    }

    fn set_ep1_tx_status_valid_dtog(&self) {
        let mut bb = self.usb_ep1r.read().bits();
        bb &= USB_EPTX_DTOGMASK;
        if (bb & 0x10) == 0 {
            bb |= 0x10
        } else {
            bb &= !0x10
        }
        if (bb & 0x20) == 0 {
            bb |= 0x20
        } else {
            bb &= !0x20
        }
        bb |= 0x1000;
        self.usb_ep1r
            .write(|w| unsafe { w.bits(bb | USB_EP_CTR_RX | USB_EP_CTR_TX) });
    }
}
