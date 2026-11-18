// Thin re-export shim. Shared source: templates/b5/lib.typ
#import "/templates/b5/lib.typ": lab-report as shared-lab-report, tabhead, tabband, tabline
#let lab-report(..args) = shared-lab-report(..args)
