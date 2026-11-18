#import "template/lib.typ": lab-report
#import "data/report.typ": report-data
#import "data/student.typ": student-data
#import "data/class.typ": class-data
#import "data/assistants.typ": assistants-data
#import "data/members.typ": members-data

#show: lab-report.with(
  ..report-data,
  student: student-data,
  class: class-data,
  assistants: assistants-data,
  members: members-data,
)

#include "content/summary.typ"
#include "content/documentation.typ"
#include "content/tools-and-materials.typ"
#include "content/tasks.typ"
#include "content/references.typ"
