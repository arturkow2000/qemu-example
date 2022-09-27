use tock_registers::{
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

register_bitfields! [
    u8,
    pub RSR [
        FE OFFSET(0) NUMBITS(1),
        PE OFFSET(1) NUMBITS(1),
        BE OFFSET(2) NUMBITS(1),
        OE OFFSET(3) NUMBITS(1),
    ],
    pub LCR [
        BRK OFFSET(0) NUMBITS(1),
        PEN OFFSET(1) NUMBITS(1),
        EPS OFFSET(2) NUMBITS(1),
        STP2 OFFSET(3) NUMBITS(1),
        FEN OFFSET(4) NUMBITS(1),
        WLEN OFFSET(5) NUMBITS(2),
        SPS OFFSET(7) NUMBITS(1),
    ]
];

register_bitfields! [
    u16,
    pub DR [
        DATA OFFSET(0) NUMBITS(8),
        FE OFFSET(8) NUMBITS(1),
        PE OFFSET(9) NUMBITS(1),
        BE OFFSET(10) NUMBITS(1),
        OE OFFSET(11) NUMBITS(1),
    ],
    pub FR [
        CTS 0,
        DSR 1,
        DCD 2,
        BUSY 3,
        RXFE 4,
        TXFF 5,
        RXFF 6,
        TXFE 7,
        RI 8
    ],
    pub CR [
        UARTEN 0,
        SIREN 1,
        SIRLP 2,
        LBE 7,
        TXE 8,
        RXE 9,
        DTR 10,
        RTS 11,
        OUT1 12,
        OUT2 13,
        RTSEN 14,
        CTSEN 15
    ],
    pub DMACR [
        RXDMAE 0,
        TXDMAE 1,
        DMAONERR 2
    ]
];

register_structs! {
    pub Registers {
        (0x000 => pub dr: ReadWrite<u16, DR::Register>),
        (0x002 => _reserved0),
        (0x004 => pub rsr: ReadWrite<u8, RSR::Register>),
        (0x005 => _reserved1),
        (0x018 => pub fr: ReadOnly<u16, FR::Register>),
        (0x01a => _reserved2),
        (0x024 => pub ibrd: ReadWrite<u16>),
        (0x026 => _reserved3),
        (0x028 => pub fbrd: ReadWrite<u16>),
        (0x02a => _reserved4),
        (0x02c => pub lcr: ReadWrite<u8, LCR::Register>),
        (0x02d => _reserved5),
        (0x030 => pub cr: ReadWrite<u16, CR::Register>),
        (0x032 => _reserved6),
        (0x034 => pub ifls: ReadWrite<u8>),
        (0x035 => _reserved7),
        (0x048 => pub dmacr: ReadWrite<u8, DMACR::Register>),
        (0x049 => @END),
    }
}
