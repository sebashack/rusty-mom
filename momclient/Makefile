BUILD_DIR:=${CURDIR}/_build
VENV_DIR:=${CURDIR}/.env
PYBIN:=${CURDIR}/.env/bin/python
PROTO_DIR:=${CURDIR}/../protos

.PHONY: set-py-venv
set-py-venv:
	rm -rf ${VENV_DIR}
	python3 -m venv ${VENV_DIR}
	${PYBIN} -m pip install -U pip wheel setuptools

.PHONY: set-py-venv-with-deps
set-py-venv-with-deps: set-py-venv
	${VENV_DIR}/bin/pip install -r requirements.txt

.PHONY: build-proto
build-proto:
	rm -rf ${BUILD_DIR}/proto_grpc
	mkdir -p ${BUILD_DIR}/proto_grpc
	${PYBIN} -m grpc_tools.protoc -I${PROTO_DIR} --python_out=${BUILD_DIR}/proto_grpc --pyi_out=${BUILD_DIR}/proto_grpc --grpc_python_out=${BUILD_DIR}/proto_grpc messages.proto
	rm -rf ${CURDIR}/lib/*_pb2*
	cp -a ${BUILD_DIR}/proto_grpc/* ${CURDIR}/lib/

.PHONY: clean
clean:
	rm -rf ${VENV_DIR}
	rm -rf ${BUILD_DIR}
	rm -rf ${CURDIR}/lib/__pycache__
	rm -rf ${CURDIR}/lib/*_pb2*
