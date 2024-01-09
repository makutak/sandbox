# コンパイラの指定
CXX := g++

# コンパイル時のフラグ
CXXFLAGS := -std=c++11 -Wall -g

# インクルードパス
INCLUDES := -I./src -I/usr/include/x86_64-linux-gnu/c++/11 -I/usr/include/c++/11

# Google Testのライブラリ
GTEST_LIBS := -lgtest -lgtest_main -pthread

# ソースファイルとオブジェクトファイル
SRC_FILES := $(wildcard src/*.cpp)
OBJ_FILES := $(SRC_FILES:.cpp=.o)

# テストファイル
TEST_FILES := $(wildcard tests/*.cpp)

# テスト用実行ファイル名
TEST_EXEC := run_tests

.PHONY: all clean test

all: test

# テストのビルドと実行
test: $(TEST_EXEC)
	./$(TEST_EXEC)

# テスト実行ファイルの作成
$(TEST_EXEC): $(OBJ_FILES) $(TEST_FILES)
	$(CXX) $(CXXFLAGS) $(INCLUDES) $^ $(GTEST_LIBS) -o $@

# オブジェクトファイルの作成
%.o: %.cpp
	$(CXX) $(CXXFLAGS) $(INCLUDES) -c $< -o $@

# cleanコマンド
clean:
	rm -f src/*.o $(TEST_EXEC)
