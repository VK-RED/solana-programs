import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Todo } from "../target/types/todo";
import { PublicKey } from "@solana/web3.js";
import { assert, AssertionError, expect } from "chai";
import fs from "fs";

describe("todo program", async () => {

    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);   

    const filePath = "/home/vishnu/.config/solana/id.json";
    const secret = JSON.parse(fs.readFileSync(filePath).toString()) as number[]; 
    const secretKey = Uint8Array.from(secret);

    const keypair = anchor.web3.Keypair.fromSecretKey(secretKey)
        
    const program = anchor.workspace.Todo as Program<Todo>;
    
    it("Should initialize a todo Account", async () => {

        const [PDA, bump] = getPdaAndBump(program.programId);

        await program.methods
        .initialize()
        .accounts({
            user:keypair.publicKey,
        })
        .rpc()
            
        const todosAccount = await program.account.todosAccount.fetch(PDA);
        expect(todosAccount.bump).to.be.equal(bump)
    })

    it("Should add new todos", async () =>{

        const todoList : {
            title: string;
            description: string | null;
            done: boolean;
        }[]= [
            {title:"Title 1", description:"Description 1", done:false}, 
            {title:"Title 2", description:null, done:true}, 
            {title:"Title 3", description:"Description 3", done:false},
            {title:"Title 4", description:null, done:true},
            {title:"Title 5", description:"Description 5", done:false}
        ]

        const [PDA] = getPdaAndBump(program.programId);

        for(const todo of todoList){
            await program.methods
            .addTodo(todo)
            .accounts({
                user:keypair.publicKey,
            })
            .signers([keypair])
            .rpc();
        }

        // Fetches the todos in reverse order, so reverse it
        const todos = (await program.account.todosAccount.fetch(PDA)).todos.reverse();
        
        for(let i = 0; i < 5; i++){
            const expected = todoList[i];
            const actual = todos[i];

            expect(expected.title).to.be.equal(actual.title);
            expect(expected.description).to.be.equal(actual.description);
            expect(expected.done).to.be.equal(actual.done);
        }
        
    });

    it("Should throw a Limit Reached Error", async () =>{

        try {

            const todoList : {
                title: string;
                description: string | null;
                done: boolean;
            }[]= [
                {title:"Title 1", description:"Description 1", done:false}, 
                {title:"Title 2", description:null, done:true}, 
                {title:"Title 3", description:"Description 3", done:false},
                {title:"Title 4", description:null, done:true},
                {title:"Title 5", description:"Description 5", done:false},
                {title:"Title 6", description:"Description 6", done:false}
            ]

            const promises: Promise<string>[] = [];
        
            for(const todo of todoList){
                const promise = program.methods
                .addTodo(todo)
                .accounts({
                    user:keypair.publicKey,
                })
                .signers([keypair])
                .rpc();

                promises.push(promise);
            }

            await Promise.all(promises);
            assert(false, "it should not have reached here!");
            
        } catch (_err) {
            if(_err instanceof anchor.AnchorError){
                const err: anchor.AnchorError = _err;
                expect(err.error.errorCode.code).to.be.equal("ReachedLimit");
            }
            else if(_err instanceof AssertionError){
                throw new Error(_err.message);
            }
        }
        
    })

})

const getPdaAndBump = (programId:PublicKey) => {

    const seed = "todos";
    const publicKey = anchor.AnchorProvider.env().wallet.publicKey;

    return PublicKey.findProgramAddressSync([
        anchor.utils.bytes.utf8.encode(seed),
        publicKey.toBuffer()
    ], programId);
}