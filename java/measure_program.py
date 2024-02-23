import argparse
import subprocess
import time
import psutil

def run_program(command, output_path="program_output.txt", timeout=3600000):
    """Executes the command and redirects the output to a file while monitoring CPU time and memory usage."""
    with open(output_path, "w") as output_file:
        start            = time.time()
        cpu_time         = 0
        max_memory_usage = 0
        
        process = subprocess.Popen(command, stdout=output_file, stderr=subprocess.STDOUT)
        ps_process = psutil.Process(process.pid)  # Wrap subprocess with psutil to monitor

        try:
            while process.poll() is None:
                time.sleep(1)  # Sleep to prevent spamming CPU usage checks
                cpu_times = ps_process.cpu_times()
                cpu_time = cpu_times.user + cpu_times.system
                current_memory_usage = (ps_process.memory_info().rss / 1024 )/ 1024  # Convert to KB
                if current_memory_usage > max_memory_usage:
                    max_memory_usage = current_memory_usage
            end = time.time()
        except subprocess.TimeoutExpired:
            print(f"Timeout expired. Program was killed after {timeout} seconds.")
            return None

        return {
            "time_taken": end - start,
            "cpu_time": cpu_time,
            "memory_usage": max_memory_usage
        }

def main(command, repetitions=1):
    """Executes the command for multiple repetitions and calculates the average performance."""
    total_time         = 0
    total_cpu_time     = 0
    total_memory_usage = 0

    for _ in range(repetitions):
        result = run_program(command, timeout=120)

        if result:
            total_time += result['time_taken']
            total_cpu_time += result['cpu_time']
            total_memory_usage += result['memory_usage']

    average_time         = total_time / repetitions
    average_cpu_time     = total_cpu_time / repetitions
    average_memory_usage = total_memory_usage / repetitions

    print(f"Average time: {average_time:.2f} sec")
    print(f"Average CPU time: {average_cpu_time:.2f} sec")
    print(f"Average memory usage: {average_memory_usage} KB")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run a command and check its performance.")
    parser.add_argument('command', type=str, nargs='+', help="Command to be executed.")
    
    args = parser.parse_args()
    command = args.command if len(args.command) > 1 else args.command[0]
    
    main(command.split())