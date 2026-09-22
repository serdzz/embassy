/* Memory map of the MSP430FR4133: 15 kB of FRAM, 2 kB of SRAM. */
MEMORY
{
  RAM     : ORIGIN = 0x2000, LENGTH = 0x0800  /* 0x2000 ..= 0x27FF */
  ROM     : ORIGIN = 0xC400, LENGTH = 0x3B80  /* 0xC400 ..= 0xFF7F */
  /* 0xFF80 ..= 0xFF87 is left alone: it holds the JTAG/BSL signatures. */
  VECTORS : ORIGIN = 0xFF88, LENGTH = 0x0078  /* 59 interrupt vectors + the reset vector */
}
