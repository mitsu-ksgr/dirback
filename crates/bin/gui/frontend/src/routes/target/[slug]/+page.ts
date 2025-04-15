/**
 *  routes/target/[slug]/+page.ts
 */

export function load({ params }) {
  return { target_id: params.slug };
}
