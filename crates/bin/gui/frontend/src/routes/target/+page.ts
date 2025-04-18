/**
 *  routes/target/[slug]/+page.ts
 */
import type { PageLoad } from "./$types";

export const load: PageLoad = ({ url }) => {
  const target_id: string | null = url.searchParams.get("target_id");

  if (target_id === null) {
    throw new Error("target_id is required");
  }

  return { target_id };
};
