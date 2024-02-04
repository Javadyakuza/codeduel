# Workflow
The Work is as follows : 

1. A `rival` submits a question to the platform by providing the following parameters: 
   
    - question_title => the question title.
    - question_body => the description of the question.
    - deadline => the deadline timestamp that the question is valid until.
    - question_status => is the question open or closed (`is a constant value of 1 for initializing the question`).
    - daredevil => which daredevil have solved the question (`is a constant value of 0 for initializing the question`).
    - category => the question category (`"Rust" only available at the moment`).
    - reward => The token amount as the question reward(`only SOL is supported at the moment`).
    - entrance_fee => The token amount as the question entrance fee(`only SOL is supported at the moment`).
    - executable_solution => This is a code snippet as sample executable solution ensuring the question is actually having an answer.
    - solution_executer => This code snippet will actually execute the executable code. this code will execute the solutions provide by the daredevils as well.
    - test_inputs => A stringified json object containing the test inputs of the executer code. (`not practical at the moment`)
    - test_outputs => A stringified json object containing the test inputs of the executer code(`not practical at the moment`).

2. after the rival provided this parameters, the rival will be prompted to send the reward amount of SOL token to the platforms account, in here the platforms acts as a prize pool. after this the question will be visible other user.

3. A daredevil can write a Executable code that will be executed by the executer code provided by the rival and the result of executing the code will returned once its executed in the backend. if the response was correct, the question status will be set to `CLOSED_SOLVED`, the `reward + entrance_fee - 1%`  will be transferred to the daredevil and all of the entrance fees left in the prize pool will be funded back to the rival.   

The parameters that must provided by the daredevil for submitting or trying a response is as follows:  

- question_id => which question id is dared evil trying to try a solution for.
- executable_solution => The actual executable code.

### And thats all. 










