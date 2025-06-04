import { NostrEvent } from "./NostrEvent.js"
import { Filter } from "./NostrMessages.js"

/**
 * Filter matching logic according to NIP-01
 */
export class NostrFilter {
  /**
   * Check if an event matches a single filter
   */
  static matchesFilter(event: NostrEvent, filter: Filter): boolean {
    // Check ids filter
    if (filter.ids && filter.ids.length > 0) {
      if (!filter.ids.includes(event.id)) {
        return false
      }
    }

    // Check authors filter
    if (filter.authors && filter.authors.length > 0) {
      if (!filter.authors.includes(event.pubkey)) {
        return false
      }
    }

    // Check kinds filter
    if (filter.kinds && filter.kinds.length > 0) {
      if (!filter.kinds.includes(event.kind)) {
        return false
      }
    }

    // Check since filter
    if (filter.since !== undefined) {
      if (event.created_at < filter.since) {
        return false
      }
    }

    // Check until filter
    if (filter.until !== undefined) {
      if (event.created_at > filter.until) {
        return false
      }
    }

    // Check tag filters (e.g., #e, #p)
    for (const [key, values] of Object.entries(filter)) {
      if (key.startsWith('#') && key.length === 2) {
        const tagName = key[1]
        const filterValues = values as string[]
        
        if (filterValues && filterValues.length > 0) {
          // Get all values for this tag name from the event
          const eventTagValues = event.getTagValues(tagName)
          
          // Check if any event tag value matches any filter value
          const hasMatch = eventTagValues.some(eventValue => 
            filterValues.includes(eventValue)
          )
          
          if (!hasMatch) {
            return false
          }
        }
      }
    }

    return true
  }

  /**
   * Check if an event matches any of the filters (OR logic)
   */
  static matchesAnyFilter(event: NostrEvent, filters: Filter[]): boolean {
    return filters.some(filter => NostrFilter.matchesFilter(event, filter))
  }

  /**
   * Validate that a filter is well-formed
   */
  static validateFilter(filter: Filter): string[] {
    const errors: string[] = []

    // Check that hex values are exactly 64 characters
    if (filter.ids) {
      for (const id of filter.ids) {
        if (!/^[a-f0-9]{64}$/.test(id)) {
          errors.push(`Invalid event ID format: ${id}`)
        }
      }
    }

    if (filter.authors) {
      for (const author of filter.authors) {
        if (!/^[a-f0-9]{64}$/.test(author)) {
          errors.push(`Invalid pubkey format: ${author}`)
        }
      }
    }

    // Check tag filters
    for (const [key, values] of Object.entries(filter)) {
      if (key.startsWith('#') && key.length === 2) {
        const tagName = key[1]
        if (!/^[a-zA-Z]$/.test(tagName)) {
          errors.push(`Invalid tag filter: ${key}`)
        }

        const filterValues = values as string[]
        if (tagName === 'e' || tagName === 'p') {
          // e and p tags should contain 64-char hex values
          for (const value of filterValues) {
            if (!/^[a-f0-9]{64}$/.test(value)) {
              errors.push(`Invalid ${tagName} tag value format: ${value}`)
            }
          }
        }
      }
    }

    // Check time bounds make sense
    if (filter.since && filter.until && filter.since > filter.until) {
      errors.push(`Invalid time range: since (${filter.since}) > until (${filter.until})`)
    }

    return errors
  }

  /**
   * Check if a filter is for replaceable events only
   */
  static isReplaceableFilter(filter: Filter): boolean {
    if (!filter.kinds || filter.kinds.length === 0) {
      return false
    }

    return filter.kinds.every(kind => {
      return (kind >= 10000 && kind < 20000) || kind === 0 || kind === 3
    })
  }

  /**
   * Check if a filter is for addressable events only
   */
  static isAddressableFilter(filter: Filter): boolean {
    if (!filter.kinds || filter.kinds.length === 0) {
      return false
    }

    return filter.kinds.every(kind => kind >= 30000 && kind < 40000)
  }

  /**
   * Extract tag filters from a filter object
   */
  static getTagFilters(filter: Filter): Record<string, string[]> {
    const tagFilters: Record<string, string[]> = {}
    
    for (const [key, values] of Object.entries(filter)) {
      if (key.startsWith('#') && key.length === 2) {
        const tagName = key[1]
        tagFilters[tagName] = values as string[]
      }
    }
    
    return tagFilters
  }

  /**
   * Normalize a filter by sorting arrays and removing empty values
   */
  static normalizeFilter(filter: Filter): Partial<Filter> {
    const normalized: any = {}

    if (filter.ids && filter.ids.length > 0) {
      normalized.ids = [...filter.ids].sort()
    }

    if (filter.authors && filter.authors.length > 0) {
      normalized.authors = [...filter.authors].sort()
    }

    if (filter.kinds && filter.kinds.length > 0) {
      normalized.kinds = [...filter.kinds].sort((a, b) => a - b)
    }

    if (filter.since !== undefined) {
      normalized.since = filter.since
    }

    if (filter.until !== undefined) {
      normalized.until = filter.until
    }

    if (filter.limit !== undefined) {
      normalized.limit = filter.limit
    }

    // Copy tag filters
    for (const [key, values] of Object.entries(filter)) {
      if (key.startsWith('#') && key.length === 2 && Array.isArray(values)) {
        if (values.length > 0) {
          normalized[key] = [...values].sort()
        }
      }
    }

    return normalized
  }
}