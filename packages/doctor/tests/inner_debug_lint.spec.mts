



import { expect, test } from 'vitest'
import { unSafeInnerDebugLint, initializeLogger } from '../es/index.mjs'
// import { spawnSync } from "node:child_process"


// test('test', async () => {
//   const res = spawnSync("node --experimental-strip-types --no-warnings packages/doctor/src/cli.mts --cwd packages/doctor/tests/fixtures",{
//     stdio: "inherit",
//     encoding:"utf8"
//   })
//   console.log(res)
//   expect(res.status).toBe(1)
// })


test('adds 1 + 2 to equal 3', async () => {
  initializeLogger()
  const res = await unSafeInnerDebugLint(
    JSON.stringify({
      env: {
        amd: true,
        node: true,
        es2021: true,
        browser: true,
        commonjs: true,
        es6: true,
        jest: true,
        mocha: true,
      },
      globals: {
        CKEDITOR: 'readonly',
        ROOT_PATH: 'readonly',
        __ROOT_SAGA__: 'readonly',
        __ROOT_REDUCER__: 'readonly',
        __ROOT_REDUX_DEVTOOLS__: 'readonly',
        __ROOT_ROUTE__: 'readonly',
        arguments: 'readonly',
      },
      rules: {
        'constructor-super': 'error',
        'for-direction': 'error',
        'getter-return': ['error', { allowImplicit: true }],
        'no-async-promise-executor': 'error',
        'no-case-declarations': 'error',
        'no-class-assign': 'error',
        'no-compare-neg-zero': 'error',
        'no-cond-assign': 'error',
        'no-const-assign': 'error',
        'no-constant-binary-expression': 'error',
        'no-constant-condition': 'error',
        'no-control-regex': 'error',
        // 'no-debugger': 'error',
        'no-delete-var': 'error',
        'no-dupe-class-members': 'error',
        'no-dupe-else-if': 'error',
        'no-dupe-keys': 'error',
        'no-duplicate-case': 'error',
        'no-empty': ['error', { allowEmptyCatch: true }],
        'no-empty-character-class': 'error',
        'no-empty-pattern': 'error',
        'no-ex-assign': 'error',
        'no-fallthrough': 'error',
        'no-func-assign': 'error',
        'no-global-assign': 'error',
        'no-import-assign': 'error',
        'no-inner-declarations': 'error',
        'no-invalid-regexp': 'error',
        'no-irregular-whitespace': 'error',
        'no-loss-of-precision': 'error',
        'no-new-native-nonconstructor': 'error',
        'no-nonoctal-decimal-escape': 'error',
        'no-obj-calls': 'error',
        'no-prototype-builtins': 'error',
        'no-redeclare': 'error',
        'no-regex-spaces': 'error',
        'no-self-assign': 'error',
        'no-setter-return': 'error',
        'no-shadow-restricted-names': 'error',
        'no-sparse-arrays': 'error',
        'no-this-before-super': 'error',
        // 'no-undef': 'error',
        'no-unexpected-multiline': 'error',
        'no-unreachable': 'error',
        'no-unsafe-finally': 'error',
        'no-unsafe-negation': ['error', { enforceForOrderingRelations: true }],
        'no-unsafe-optional-chaining': 'error',
        'no-unused-labels': 'error',
        // 'no-unused-vars': 'error',
        'no-useless-catch': 'error',
        'no-useless-escape': 'error',
        // 'no-with': 'error',
        'use-isnan': ['error', { enforceForIndexOf: true }],
        'valid-typeof': 'error',

        // 'require-atomic-updates': 'error',
        // 'use-before-define': 'error',

        // 'react/jsx-key': 'error',
        'react/jsx-no-comment-textnodes': 'error',
        'react/jsx-no-duplicate-props': 'error',
        'react/jsx-no-target-blank': 'error',
        'react/jsx-no-undef': 'error',
        'react/no-children-prop': 'error',
        'react/no-danger-with-children': 'error',
        'react/no-direct-mutation-state': 'error',
        'react/no-find-dom-node': 'error',
        'react/no-is-mounted': 'error',
        'react/no-render-return-value': 'error',
        'react/no-string-refs': 'error',
        'react/no-unescaped-entities': 'error',
        // 'react/react-in-jsx-scope': 'error',
        'react/require-render-return': 'error',
        // 'react/rules-of-hooks': 'error',

        'import/export': 'error',

        // 'unicorn/filename-case': 'error',
        'unicorn/new-for-builtins': 'error',
        'unicorn/no-instanceof-array': 'error',
        'unicorn/no-invalid-remove-event-listener': 'error',
        'unicorn/no-thenable': 'error',
        'unicorn/no-unreadable-array-destructuring': 'error',
        'unicorn/require-array-join-separator': 'error',
        'unicorn/require-number-to-fixed-digits-argument': 'error',

        '@typescript-eslint/no-duplicate-enum-values': 'error',
        '@typescript-eslint/no-extra-non-null-assertion': 'error',
        '@typescript-eslint/no-misused-new': 'error',
        '@typescript-eslint/no-non-null-asserted-optional-chain': 'error',
        '@typescript-eslint/no-unsafe-declaration-merging': 'error',
        '@typescript-eslint/no-unsafe-function-type': 'error',
        '@typescript-eslint/no-wrapper-object-types': 'error',
        '@typescript-eslint/prefer-namespace-keyword': 'error',
      },
    }),
    {
      cwd: 'tests/fixtures',
      verbose: false,
      
    },
  )

  
  expect(res).toBeDefined()

  expect(res.length).toBe(1)
})
