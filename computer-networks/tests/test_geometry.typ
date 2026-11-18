#import "../template/lib.typ": lab-report

#show: lab-report.with(
  session: 1,
  title: "Test Geometry",
  date: "September 2026",
  student: (name: "Test Student", id: "12345678"),
  class: "RA",
  assistants: ("Asisten 1", "Asisten 2")
)

= Test Section Heading
This is a test paragraph verifying B5 geometry and double-rule header rendering.
