# Hardware

The Live Signage system requires some assembly and bespoke component fabrication. Luckily, if one has access to a laser cutter at their local library or makerspace, this process becomes much easier. This tutorial is ment to be read in its entrirety before the first step is taken. Sign installation sites will need to be evaluated for feasibility before installation of the power delivery system and sign anchor points.

## Gather These First

### Materials Needed

- Opaque Acrylic (Color choice can be used for branding. We used black for a clean look.) at 12" x 19" at 1/8th"

- Clear Acrylic (For replacable screen cover.) also at 12" x 19" at 1/8th

- Glue (G25?) Before using this glue, read the detailed discription below. It does not act like normal super glue!

- KYY 15.6" FHD 1080p Portable Laptop Monitor with USB-C and Mini HDMI Support

- Raspberry Pi 0 2 W (This MUST be the 2nd Edition of the Pi0W. The original Pi0W will not have enough resources to run this system! We tried!)

- 65W USB-C Laptop Charger

- 180° Angled Mini HDMI Male to HDMI Female Adapter

- 1' 90° USB C to Micro USB OTG Cable

- 15cm 90° Right Angle Mini HDMI Male to HDMI 2.0 Male Patch Cable

- 11 x 11 x 5.3 mm Anodized Aluminum Heat Sink (You can buy a pack of assorted heat sinks)

- Radio-remote Controlled Power Outlet Adapters. I recommend the Syantek 5 pack on Amazon, as you can have up to 5 seperate live signs on one remote, with a backup to give front facing employees.
  These should be on a band that doesn't have a lot of interference in your area. We use around 430 MHz.

- 1/2" Wood For Frame. If you plan on laser cutting this wood you will need to purchase it from a craft store and not a hardware store. Hardware stores treat their wood with flame-retardant and it makes
  it very difficult to create consistant cuts. You can also use laser cut pieces of 1/16ths inch craft wood that you can then layer, or just CNC the the 1/2' wood frame.

- A Micro SD Card (sometimes called a TF Card) of at least 8Gb from a reputable brand. Almost all the issues we have had with long term system stability can be directly contributed to using cheap after
  market micro SD cards. Use a PNY, Kingston, Samsung, etc.

- 1/4" - 20 Drywall Anchors (The length varies with the depth of your drywall. Ours was 1/2")

### Tools Needed
- A Computer with Windows
- Nitrile Gloves
- Isopropanol in a Spray Bottle
- Clear Cellophane Tape
- A 1/4" Drill Bit Rated for Metal
- A Drywall Saw
- A Drill
- A Drill Bit Set with Phillips Heads
- Washers with a 1/4" Diameter Hole
- A Can of Compressed Air or An Electric Duster
- A Cable Snake
- Electrical Tape
- 3D Printer Filament Snips (A Great Universal Tool!)
- Plastic Rope for Pulling Cable
- A MicroSD to USB Adapter
- A Fan of Some Sort (I used a USB Desk Fan)
- A Ladder
- Another Person

### Software and Files Needed

- Raspberry Pi Imager
- MiniTool Partition Wizard
- Laser Cutter Software (We used LightBurn)
- LightBurn Cut Files OR use provided SVG file with your own laser cutter software
- Putty (Or any SSH program)


## The Process


### Creating Live Sign Units

1. First thing that you need to do is prepare the Operating System for the Live Signs.

2. Insert the MicroSD card into your MicroSD to USB converter and then plug it into your PC.

3. Open MiniTool Partition Wizard and check to make sure you don't have any superfluous partitions. If you do, remove them. After this format the whole card to Fat32. Record the Drive Letter of this new partition.

4. Open Raspberry Pi Imager and hold down Ctrl + Shift + X. This opens the OS Customization Window. Here, configure your signs unique hostname, enter your WiFi credentials, Wireless LAN country, and Enable SSH in the Services Tab. Be sure to remember this password!

5. Then select the Pi 0 2 W as your Board, Select Raspberry Pi OS (other) and in the subsequent menu select Raspberry Pi OS Lite 64-Bit. After that select your Storage Device and then select your MicroSD Card. Confirm that its drive letter is the one you recorded earlier!

6. Once all of these configuration steps have been completed, go ahead and hit next and confirm the write.

7. After the OS has been flashed, slot it into the Raspbery Pi 0 2 W's Micro SD Card slot.

8. Insert the 180° Mini HDMI Male to HDMI Female adapter into the KYY Portable Monitor.

9. Attach the HDMI Patch Cable to the Adapter

10. Attach the other end of the HDMI Patch Cable to the Raspberry Pi's Mini HDMI port.

11. Attach the right angle USB-C Patch Cable to the KYY Portable Monitor on the outside USB-C Port.

12. Attach the other end of the USB-C to Mini USB Patch cable to the PWR Port of the Raspberry Pi.

13. Secure the Pi to the back of the KYY Monitor by using electrical tape over the patch cables.
![It Should Look Like This](SignBack.jpg)

14. After that, Plug in the 65W Laptop Charger and allow the Pi to boot. Follow the instructions on the Software side of things in the Main Repo and confirm that your Live Sign is working correctly. I recommend making an image of this working Install so that you can quickly reflash new SD cards if there is ever a hardware failure. 


### Creating Frames and Covers

## Installation

### Preparing The Area

