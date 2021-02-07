import assert from 'https://jspm.dev/webassert@3.0'
import init, { parse, StaticImport, DynamicImport, ImportMeta, Export } from "../wasm_web/es_module_lexer.js"

describe("es-modules-lexer", () => {
    describe("init()", () => {
        it("initializes wasm module",async () => {
           const start = performance.now()
           await init()
           const duration = performance.now() - start;
           console.log('init took > ', duration, 'ms')
        })
    })

    describe("parse()", () => {
        it("parses imports", () => {
            const source = `
                import { foo } from "bar";
                import("./dynamicModule.js")
                console.log(import.meta.url)
            `
            const { imports } = parse(source)

            assert(imports[0] instanceof StaticImport)
            {
                const {start, end} = imports[0].statementRange()
                assert(source.substring(start, end) === 'import { foo } from "bar"')
            }
            {
                const {start, end} = imports[0].moduleSpecifierRange()
                assert(source.substring(start, end) === "bar")
            }

            assert(imports[1] instanceof DynamicImport)
            {
                const {start, end} = imports[1].importExpressionRange()
                assert(source.substring(start, end) === 'import("./dynamicModule.js")')
            }
            {
                const {start, end} = imports[1].moduleSpecifierExpressionRange()
                assert(source.substring(start, end) === '"./dynamicModule.js"')
            }

            assert(imports[2] instanceof ImportMeta)
            {
                const { start, end } = imports[2].expressionRange()
                assert(source.substring(start, end) === "import.meta")
            }
        })

        it("parses exports", () => {
            const source = `
                const much = "";
                function fun() {}
                
                export { much, fun }
                export const foo = 42;
            `
            const { exports } = parse(source)

            assert(exports[0] instanceof Export)
            {
                const { start, end } = exports[0].exportSpecifierRange()
                assert(source.substring(start, end) === "much")
            }

            assert(exports[1] instanceof Export)
            {
                const { start, end } = exports[1].exportSpecifierRange()
                assert(source.substring(start, end) === "fun")
            }

            assert(exports[2] instanceof Export)
            {
                const { start, end } = exports[2].exportSpecifierRange()
                assert(source.substring(start, end) === "foo")
            }
        })
    })
})
