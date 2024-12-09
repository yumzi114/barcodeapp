#!/bin/bash
check_internet() {
  ping -c 1 8.8.8.8 > /dev/null 2>&1
  return $?  # ping 명령의 종료 상태 코드 반환
}

while ! check_internet; do
  echo "인터넷에 연결되지 않았습니다. 잠시 후 다시 시도합니다..."
  sleep 1  # 5초 대기 후 다시 시도
done

echo "인터넷에 연결되었습니다."

startx /home/yum/runner
