MEMORY
{
  FLASH : ORIGIN = 0x00000000 + 0x27000, LENGTH = 1024K - 0x27000
  RAM   : ORIGIN = 0x20000000 + 0x0f588, LENGTH =  256K - 0x0f588

  /*
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  RAM   : ORIGIN = 0x20000000, LENGTH = 128K
  */

}