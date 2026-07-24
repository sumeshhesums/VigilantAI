"use client";

import { useRoles } from "@/hooks/use-roles";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { EmptyState } from "@/components/shared/empty-state";
import { ErrorState } from "@/components/shared/error-state";
import { TableSkeleton } from "@/components/shared/loading-skeleton";

const roleColorMap: Record<string, string> = {
  system_admin: "bg-red-500/20 text-red-400 border-red-500/30",
  security_admin: "bg-orange-500/20 text-orange-400 border-orange-500/30",
  security_analyst: "bg-blue-500/20 text-blue-400 border-blue-500/30",
  operator: "bg-emerald-500/20 text-emerald-400 border-emerald-500/30",
  viewer: "bg-gray-500/20 text-gray-400 border-gray-500/30",
};

export function RoleTable() {
  const { data: roles, isLoading, error, refetch } = useRoles();

  if (isLoading) return <TableSkeleton />;
  if (error) return <ErrorState onRetry={refetch} />;

  if (!roles || roles.length === 0) {
    return (
      <Card>
        <CardHeader><CardTitle>Roles</CardTitle></CardHeader>
        <CardContent>
          <EmptyState title="No roles found" description="No roles are configured." />
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Roles</CardTitle>
      </CardHeader>
      <CardContent>
        <div className="rounded-md border">
          <table className="w-full">
            <thead>
              <tr className="border-b bg-muted/50">
                <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Name</th>
                <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Description</th>
              </tr>
            </thead>
            <tbody>
              {roles.map((role) => (
                <tr key={role.id} className="border-b transition-colors hover:bg-muted/50">
                  <td className="px-4 py-3">
                    <Badge variant="outline" className={roleColorMap[role.name] || ""}>
                      {role.name}
                    </Badge>
                  </td>
                  <td className="px-4 py-3 text-sm text-muted-foreground">
                    {role.description || "No description"}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </CardContent>
    </Card>
  );
}
