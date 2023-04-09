#include <dmsdk/sdk.h>

extern "C" {
	// Crashes
	dmVMath::Point3 GetPositionDirectly(dmGameObject::HInstance instance) {
		return dmGameObject::GetPosition(instance);
	}

	// Works
	void GetPositionWrapper(dmGameObject::HInstance instance, dmVMath::Point3* out) {
		*out = dmGameObject::GetPosition(instance);
	}
}
