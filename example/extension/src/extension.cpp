#include <dmsdk/sdk.h>
#include <dmsdk/gameobject/component.h>

//extern "C" dmVMath::Point3 pos(dmGameObject::HInstance instance)
//{
//	//dmGameObject::HInstance instance = dmScript::CheckGOInstance(L);
//	dmVMath::Point3 pos = dmGameObject::GetPosition(instance);

	//dmScript::PushVector3(L, dmVMath::Vector3(pos));

//	return pos;
//}

extern "C" int CGetPos(lua_State* L) {
	DM_LUA_STACK_CHECK(L, 1);

	dmGameObject::HInstance instance = dmScript::CheckGOInstance(L);
	dmVMath::Point3 position = dmGameObject::GetPosition(instance);
	dmScript::PushVector3(L, dmVMath::Vector3(position));

	return 1;
}

static dmGameObject::Result Create(const dmGameObject::ComponentTypeCreateCtx* ctx, dmGameObject::ComponentType* type) {
	return dmGameObject::RESULT_OK;
}


//DM_DECLARE_COMPONENT_TYPE(MY_COMPONENT, "thing", Create, 0);