# Project 2

Throughout the semester I'm making a ticket booking service, basically TicketMaster but better. For this particular project, I will be using examples from my ticket booking service to demonstrate understanding of particular concepts of Rust Data Representation. 

WHat Ill be using: 
- an unsigned 16-bit integer (u16) that stores:
  - bit 0: activity (0=inactive, 1=active)
  - bit 1-2: role (00= viewer, 01=buyer, 10=seller, 11=admin)
  - bit 3-7: event id (event that user is viewing/buying/selling)
  - bit 8-14: seats remaining
  - bit 15: endianness (0=little-endian, 1=big-endian)

- functions that extract bits using bit shifting and return a u16:
  - get_activity
  - get_role
  - get_session_id
  - get_seats_remaining
  - get endinaness

- In one or more of your functions, read your designated endian bit to determine the format. Use that bit's value to decide whether to convert a specific extracted flag into big-endian format before returning it as the output.





# Formal Instructions:

Please accomplish the to-dos following the sequential order (from the top to the bottom). It's important to go through the instructional materials before attempting the assessment activities. 

Reading 

Chapter 3.2: Data TypesLinks to an external site.

Appendix B: Operators and SymbolsLinks to an external site. (for bitwise operators)

Homework: practice bit operations and endian conversion. 

Requirement:

System State: Create one unsigned integer variable to represent the state of your system. Define the meaning of the bit flags within that variable for your specific domain (e.g., bit 0 represents "is_active", bit 1 represents "is_admin", bits 2-8 represent "group_id").

Designate exactly one bit within this variable to represent the machine's endianness (e.g., 0 for little-endian, 1 for big-endian). 

Write function(s) that extract specific bit(s) from the unsigned variable in order to access and read your defined flags.

In one or more of your functions, read your designated endian bit to determine the format. Use that bit's value to decide whether to convert a specific extracted flag into big-endian format before returning it as the output.

Submission Requirement: Please submit the link to a public Git repository containing your code for this assignment.

Grading 
The grade for this assignment will be based on your presentation in our next class. The rubric is as follows:

90%+ – Provides good examples for discussion and demonstrates an great understanding of the assigned chapters.

80%+ – Provides good examples for discussion and demonstrates a fair understanding of the assigned chapters.

70%+ – Provides fair examples for discussion and demonstrates an great understanding of the assigned chapters.

60%+ – Provides fair examples for discussion and demonstrates a fair understanding of the assigned chapters.

50%+ – Provides examples for discussion but fails to demonstrate understanding of the assigned chapters.

0% – No examples provided.

# cs-h230-project2
