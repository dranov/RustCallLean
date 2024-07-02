import Lake
open System Lake DSL

package callee

@[default_target]
lean_lib Callee where
  defaultFacets := #[LeanLib.sharedFacet]
