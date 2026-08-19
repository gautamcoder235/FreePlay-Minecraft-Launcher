# Design Tokens Engine & Build Pipeline

---

## 1. Overview

Design tokens are maintained in single-source-of-truth JSON files (`tokens.json`) following the W3C DTCG standard and transformed into target outputs (CSS Variables, Tailwind Config, TypeScript types, Swift/Android tokens) via Style Dictionary.

---

## 2. Style Dictionary Build Pipeline

```javascript
// Style Dictionary Configuration Sample
module.exports = {
  source: ['tokens/**/*.json'],
  platforms: {
    css: {
      transformGroup: 'css',
      buildPath: 'build/css/',
      files: [{
        destination: 'variables.css',
        format: 'css/variables'
      }]
    },
    ts: {
      transformGroup: 'js',
      buildPath: 'build/ts/',
      files: [{
        destination: 'tokens.ts',
        format: 'javascript/es6'
      }]
    }
  }
};
```

---

## 3. Key References

- [Style Dictionary Engine Documentation](https://amzn.github.io/style-dictionary/)
- [W3C Design Tokens Community Group (DTCG)](https://www.designtokens.org/)
