/* Adafruit Feather nRF52832 with the Adafruit nRF52 bootloader:
 *
 *   0x00000000  MBR + SoftDevice S132 (kept by the bootloader, unused here)
 *   0x00026000  application
 *   0x00074000  bootloader
 *   0x0007F000  bootloader settings
 *
 * The SoftDevice is never enabled, so all of RAM is ours.
 */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00026000, LENGTH = 0x74000 - 0x26000
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
}
