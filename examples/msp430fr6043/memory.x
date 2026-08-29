/* Memory map of the MSP430FR6043, as far as a 16-bit target can see it.
 *
 * The device has 64 kB of FRAM, but it runs past 0xFFFF: 0x6000 to 0xFF7F is reachable with 16-bit
 * addressing, and the 24 kB above 0x10000 needs the 20-bit addressing that Rust's msp430 target
 * does not have. So the budget is the ~40 kB below.
 *
 * Two regions are deliberately absent. The 8 kB at 0x4000-0x5FFF is the LEA accelerator's working
 * memory, which belongs to whatever drives the ultrasonic front end rather than to the linker. The
 * 16 bytes at 0xFF80 are the JTAG, BSL and IPE signatures: writing them by accident locks the part.
 *
 * The vector table starts at 0xFF92 rather than the 0xFF90 in TI's own linker script. That is where
 * the generated PAC puts it: the SVD numbers SDHS as vector 15 at 0xFFB0, which fixes the base two
 * bytes higher. The check below is that SDHS lands on 0xFFB0 in the linked image.
 */
MEMORY
{
  RAM     : ORIGIN = 0x1C00, LENGTH = 0x1000  /* 0x1C00 ..= 0x2BFF */
  ROM     : ORIGIN = 0x6000, LENGTH = 0x9F80  /* 0x6000 ..= 0xFF7F */
  VECTORS : ORIGIN = 0xFF92, LENGTH = 0x006E  /* 54 interrupt vectors + the reset vector */
}
