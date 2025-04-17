/**
 *  routes/target/[slug]/+page.ts
 */
import type { PageLoad } from './$types';

export function load({ url }): PageLoad {
  const target_id = url.searchParams.get('target_id');

  if (target_id === null) {
    throw new Error('target_id is required');
  }

  return { target_id };
}
