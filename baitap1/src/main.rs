fn main() {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];

    for i in 0..org_arr.len() {
        // Kiểm tra phần tử đầu tiên của sub_array có trong org_arr

        if (sub_arr[0] == org_arr[i]) {
            /*
             * Nếu có kiểm tra tiếp các phần tử tiếp theo của sub_arr
             * có trùng với các phần tử tiếp theo của org_arr
             */

            for j in 1..sub_arr.len() {
                if (sub_arr[j] != org_arr[i + j]) {
                    println!("false {}", j);
                    return;
                }
            }
            println!("true");
            return;
        }
    }
}
