/* Memory map of the MSP430FR2355: 32 kB of FRAM, 4 kB of SRAM. */
MEMORY
{
  RAM     : ORIGIN = 0x2000, LENGTH = 0x1000  /* 0x2000 ..= 0x2FFF */
  ROM     : ORIGIN = 0x8000, LENGTH = 0x7F80  /* 0x8000 ..= 0xFF7F */
  /* 0xFF80 ..= 0xFFA3 is left alone: it holds the JTAG/BSL/IPE signatures. */
  VECTORS : ORIGIN = 0xFFA4, LENGTH = 0x005C  /* 45 interrupt vectors + the reset vector */
}
