/* Adafruit Feather nRF52832 with the Adafruit nRF52 bootloader and SoftDevice S132 6.1.1:
 *
 *   0x00000000  MBR + SoftDevice S132 6.1.1
 *   0x00026000  application
 *   0x00074000  bootloader
 *
 * The SoftDevice owns RAM below the application's RAM origin. How much it needs depends on the
 * configuration passed to `Softdevice::enable`; if it is too little, the application panics with
 * the start address it needs, and this is the line to change.
 */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00026000, LENGTH = 0x74000 - 0x26000
  RAM : ORIGIN = 0x20002c28, LENGTH = 0x20010000 - 0x20002c28
}
