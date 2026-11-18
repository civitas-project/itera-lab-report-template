#import "template/lib.typ": lab-report
#import "data/report.typ": report-data
#import "data/student.typ": student-data
#import "data/class.typ": class-data
#import "data/assistants.typ": assistants-data

#show: lab-report.with(
  ..report-data,
  student: student-data,
  class: class-data,
  assistants: assistants-data,
)

#include "content/01-tujuan.typ"
#include "content/02-alat-bahan.typ"
#include "content/03-langkah.typ"
#pagebreak()
#include "content/04-source-code.typ"

#pagebreak()
#include "content/05-output.typ"

#pagebreak()
#include "content/06-analisis.typ"

#pagebreak()
#include "content/07-kesimpulan.typ"
