/**
 * Debounce utility function
 * 
 * Creates a debounced function that delays invoking func until after wait milliseconds
 * have elapsed since the last time the debounced function was invoked.
 * 
 * @param func - The function to debounce
 * @param wait - The number of milliseconds to delay
 * @param immediate - If true, trigger the function on the leading edge instead of trailing
 * @returns A debounced version of the function
 */
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number,
  immediate: boolean = false
): T & { cancel: () => void } {
  let timeout: number | null = null;
  let result: ReturnType<T>;

  const debounced = function (this: any, ...args: Parameters<T>): ReturnType<T> {
    const context = this;
    
    const later = () => {
      timeout = null;
      if (!immediate) {
        result = func.apply(context, args);
      }
    };

    const callNow = immediate && !timeout;
    
    if (timeout) {
      clearTimeout(timeout);
    }
    
    timeout = setTimeout(later, wait);
    
    if (callNow) {
      result = func.apply(context, args);
    }
    
    return result;
  } as T & { cancel: () => void };

  // Add cancel method to clear pending execution
  debounced.cancel = () => {
    if (timeout) {
      clearTimeout(timeout);
      timeout = null;
    }
  };

  return debounced;
}

/**
 * Simple debounce utility for class methods
 * 
 * Creates a debounced wrapper that can be used within classes to prevent
 * rapid successive calls to the same method.
 * 
 * @param key - Unique identifier for the debounced function
 * @param wait - The number of milliseconds to delay
 * @param immediate - If true, trigger the function on the leading edge
 * @returns A function that wraps the original function with debouncing
 */
export function createDebounced<T extends (...args: any[]) => any>(
  key: string,
  wait: number,
  immediate: boolean = false
): (func: T) => (...args: Parameters<T>) => ReturnType<T> | undefined {
  const debounceMap = new Map<string, { timeout: number | null; result: any }>();

  return function (func: T) {
    return function (this: any, ...args: Parameters<T>): ReturnType<T> | undefined {
      const context = this;
      const debounceState = debounceMap.get(key) || { timeout: null, result: undefined };

      const later = () => {
        debounceState.timeout = null;
        if (!immediate) {
          debounceState.result = func.apply(context, args);
        }
        debounceMap.set(key, debounceState);
      };

      const callNow = immediate && !debounceState.timeout;

      if (debounceState.timeout) {
        clearTimeout(debounceState.timeout);
      }

      debounceState.timeout = setTimeout(later, wait);
      debounceMap.set(key, debounceState);

      if (callNow) {
        debounceState.result = func.apply(context, args);
        debounceMap.set(key, debounceState);
      }

      return debounceState.result;
    };
  };
}

/**
 * Decorator for debouncing class methods
 * 
 * @param wait - The number of milliseconds to delay
 * @param immediate - If true, trigger the function on the leading edge
 * @returns Method decorator
 */
export function Debounce(wait: number, immediate: boolean = false) {
  return function (_target: any, _propertyKey: string, descriptor: PropertyDescriptor) {
    const originalMethod = descriptor.value;
    const debounceMap = new Map<any, { timeout: number | null; result: any }>();

    descriptor.value = function (this: any, ...args: any[]) {
      const context = this;
      const debounceState = debounceMap.get(context) || { timeout: null, result: undefined };

      const later = () => {
        debounceState.timeout = null;
        if (!immediate) {
          debounceState.result = originalMethod.apply(context, args);
        }
        debounceMap.set(context, debounceState);
      };

      const callNow = immediate && !debounceState.timeout;

      if (debounceState.timeout) {
        clearTimeout(debounceState.timeout);
      }

      debounceState.timeout = setTimeout(later, wait);
      debounceMap.set(context, debounceState);

      if (callNow) {
        debounceState.result = originalMethod.apply(context, args);
        debounceMap.set(context, debounceState);
      }

      return debounceState.result;
    };

    return descriptor;
  };
}
