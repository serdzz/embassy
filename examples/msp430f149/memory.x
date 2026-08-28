/* Memory map of the MSP430F149: 60 kB of flash, 2 kB of SRAM. */
MEMORY
{
  RAM     : ORIGIN = 0x0200, LENGTH = 0x0800  /* 0x0200 ..= 0x09FF */
  ROM     : ORIGIN = 0x1100, LENGTH = 0xEEE0  /* 0x1100 ..= 0xFFDF */
  VECTORS : ORIGIN = 0xFFE0, LENGTH = 0x0020  /* 15 interrupt vectors + the reset vector */
}
